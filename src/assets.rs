use asefile::AsepriteFile;
use hashmap_macro::hashmap;
use image::EncodableLayout;
use macroquad::prelude::*;

use crate::utils::*;

pub struct Assets {
    pub tileset: Spritesheet,
    pub gift_selection_screen: Texture2D,
    pub win_screen: Texture2D,
    pub start_button: Texture2D,
    pub start_button_hovered: Texture2D,
    pub gift_sprites: Animation,
    pub arrow: Texture2D,
    pub arrow_hovered: Texture2D,
    pub font: Spritesheet,
}
impl Default for Assets {
    fn default() -> Self {
        Self {
            tileset: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/tileset.ase"), None),
                8.0,
            ),
            gift_selection_screen: load_ase_texture(
                include_bytes!("../assets/gift_selection_screen.ase"),
                None,
            ),
            win_screen: load_ase_texture(include_bytes!("../assets/win_screen.ase"), None),
            arrow: load_ase_texture(include_bytes!("../assets/arrow.ase"), None),
            arrow_hovered: load_ase_texture(include_bytes!("../assets/arrow_hovered.ase"), None),
            start_button: load_ase_texture(include_bytes!("../assets/start_button.ase"), None),
            start_button_hovered: load_ase_texture(
                include_bytes!("../assets/start_button_hovered.ase"),
                None,
            ),
            gift_sprites: Animation::from_file(include_bytes!("../assets/gifts.ase")),
            font: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/font.ase"), None),
                4.0,
            ),
        }
    }
}
impl Assets {
    pub fn draw_text(&self, text: &str, mut x: f32, mut y: f32) -> (f32, f32) {
        let original_x = x;
        let original_y = y;
        let hardcoded = hashmap!(':'=>36,'.'=>37,'-'=>38,'%'=>39,'+'=>40,'/'=>41,'H'=>42,'('=>43,')'=>44,'!'=>45,'?'=>46);
        gl_use_material(&COLOR_MOD_MATERIAL);
        COLOR_MOD_MATERIAL.set_uniform("color", TEXT_COLORS[1]);
        let mut start_of_line = true;

        for char in text.chars() {
            if char == '\n' {
                start_of_line = true;
                y += 5.0;
                x = original_x;
                continue;
            } else if char == ' ' {
                if start_of_line {
                    continue;
                }
                x += 4.0;
                continue;
            }
            let code = char as u8;
            if code < TEXT_COLORS.len() as u8 {
                COLOR_MOD_MATERIAL.set_uniform("color", TEXT_COLORS[code as usize]);
            }

            let index = if let Some(value) = hardcoded.get(&char) {
                *value
            } else if code.is_ascii_lowercase() {
                code - b'a'
            } else if code.is_ascii_digit() {
                code - b'0' + 26
            } else {
                continue;
            };
            start_of_line = false;
            self.font
                .draw_sprite(x + 2.0, y + 2.0, index as f32, 0.0, None);

            x += 4.0
        }

        COLOR_MOD_MATERIAL.set_uniform("color", TEXT_COLORS[0]);
        gl_use_default_material();
        (x - original_x, y - original_y)
    }
}
fn load_ase_texture(bytes: &[u8], layer: Option<u32>) -> Texture2D {
    let img = AsepriteFile::read(bytes).unwrap();
    let img = if let Some(layer) = layer {
        img.layer(layer).frame(0).image()
    } else {
        img.frame(0).image()
    };
    let new = Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.as_bytes().to_vec(),
    };
    let texture = Texture2D::from_image(&new);
    texture.set_filter(FilterMode::Nearest);
    texture
}

pub struct Spritesheet {
    pub texture: Texture2D,
    pub sprite_size: f32,
}
impl Spritesheet {
    pub fn new(texture: Texture2D, sprite_size: f32) -> Self {
        Self {
            texture,
            sprite_size,
        }
    }
    /// Same as `draw_tile`, except centered
    pub fn draw_sprite(
        &self,
        screen_x: f32,
        screen_y: f32,
        tile_x: f32,
        tile_y: f32,
        params: Option<&DrawTextureParams>,
    ) {
        self.draw_tile(
            screen_x - self.sprite_size / 2.0,
            screen_y - self.sprite_size / 2.0,
            tile_x,
            tile_y,
            params,
        );
    }
    /// Draws a single tile from the spritesheet
    pub fn draw_tile(
        &self,
        screen_x: f32,
        screen_y: f32,
        tile_x: f32,
        tile_y: f32,
        params: Option<&DrawTextureParams>,
    ) {
        let mut p = params.cloned().unwrap_or(DrawTextureParams::default());
        p.dest_size = p
            .dest_size
            .or(Some(Vec2::new(self.sprite_size, self.sprite_size)));
        p.source = p.source.or(Some(Rect {
            x: tile_x * self.sprite_size,
            y: tile_y * self.sprite_size,
            w: self.sprite_size,
            h: self.sprite_size,
        }));
        draw_texture_ex(&self.texture, screen_x, screen_y, WHITE, p);
    }
}

pub struct Animation {
    frames: Vec<(Texture2D, u32)>,
    pub total_length: u32,
}
impl Animation {
    pub fn from_file(bytes: &[u8]) -> Self {
        let ase = AsepriteFile::read(bytes).unwrap();
        let mut frames = Vec::new();
        let mut total_length = 0;
        for index in 0..ase.num_frames() {
            let frame = ase.frame(index);
            let img = frame.image();
            let new = Image {
                width: img.width() as u16,
                height: img.height() as u16,
                bytes: img.as_bytes().to_vec(),
            };
            let duration = frame.duration();
            total_length += duration;
            let texture = Texture2D::from_image(&new);
            frames.push((texture, duration));
        }
        Self {
            frames,
            total_length,
        }
    }
    pub fn get_at_time(&self, mut time: u32) -> &Texture2D {
        time %= self.total_length;
        for (texture, length) in self.frames.iter() {
            if time >= *length {
                time -= length;
            } else {
                return texture;
            }
        }
        panic!()
    }
}

pub struct World {
    pub collision: Vec<Chunk>,
    pub one_way_collision: Vec<Chunk>,
    pub details: Vec<Chunk>,
    pub background: Vec<Chunk>,
    pub interactable: Vec<Chunk>,
}
impl World {
    pub fn get_interactable_spawn(&self, tile_index: i16) -> Option<Vec2> {
        for chunk in self.interactable.iter() {
            for (i, tile) in chunk.tiles.iter().enumerate() {
                if *tile == tile_index + 1 {
                    return Some(Vec2::new(
                        (i as i16 % 16 + chunk.x) as f32 * 8.0,
                        (i as i16 / 16 + chunk.y) as f32 * 8.0,
                    ));
                }
            }
        }
        None
    }
    pub fn get_collision_chunk(&self, x: i16, y: i16) -> Option<&Chunk> {
        self.collision.iter().find(|f| f.x == x && f.y == y)
    }
    pub fn get_one_way_collision_chunk(&self, x: i16, y: i16) -> Option<&Chunk> {
        self.one_way_collision.iter().find(|f| f.x == x && f.y == y)
    }
    pub fn set_collision_tile(&mut self, x: i16, y: i16, tile: i16) {
        let cx = ((x as f32 / 16.0).floor() * 16.0) as i16;
        let cy = ((y as f32 / 16.0).floor() * 16.0) as i16;

        let chunk = self
            .collision
            .iter_mut()
            .find(|f| f.x == cx && f.y == cy)
            .unwrap();
        chunk.tiles[(x - chunk.x + (y - chunk.y) * 16) as usize] = tile;
    }
}
impl Default for World {
    fn default() -> Self {
        let xml = include_str!("../assets/tilemap/world.tmx");
        let collision = get_layer(xml, "Collision");
        let one_way_collision = get_layer(xml, "OneWayCollision");
        let detail = get_layer(xml, "Detail");
        let interactable = get_layer(xml, "Interactable");
        let background = get_layer(xml, "Background");
        World {
            collision: get_all_chunks(collision),
            one_way_collision: get_all_chunks(one_way_collision),
            details: get_all_chunks(detail),
            background: get_all_chunks(background),
            interactable: get_all_chunks(interactable),
        }
    }
}

pub struct Chunk {
    pub x: i16,
    pub y: i16,
    pub tiles: Vec<i16>,
}
impl Chunk {
    pub fn tile_at(&self, x: usize, y: usize) -> Option<i16> {
        if x > 16 {
            return None;
        }
        self.tiles.get(x + y * 16).cloned()
    }
    pub fn draw(&self, assets: &Assets) {
        for (index, tile) in self.tiles.iter().enumerate() {
            if *tile == 0 {
                continue;
            }
            let tile = *tile - 1;
            let x = index % 16;
            let y = index / 16;
            assets.tileset.draw_tile(
                self.x as f32 * 8.0 + (x * 8) as f32,
                self.y as f32 * 8.0 + (y * 8) as f32,
                (tile % 32) as f32,
                (tile / 32) as f32,
                None,
            );
        }
    }
}

fn get_all_chunks(xml: &str) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let mut xml = xml.to_string();
    while let Some((current, remains)) = xml.split_once("</chunk>") {
        let new = parse_chunk(current);
        chunks.push(new);
        xml = remains.to_string();
    }

    chunks
}

fn get_layer<'a>(xml: &'a str, layer: &str) -> &'a str {
    let split = format!(" name=\"{layer}");
    xml.split_once(&split)
        .unwrap()
        .1
        .split_once(">")
        .unwrap()
        .1
        .split_once("</layer>")
        .unwrap()
        .0
}

fn parse_chunk(xml: &str) -> Chunk {
    let (tag, data) = xml
        .split_once("<chunk ")
        .unwrap()
        .1
        .split_once(">")
        .unwrap();

    let x = tag
        .split_once("x=\"")
        .unwrap()
        .1
        .split_once("\"")
        .unwrap()
        .0
        .parse()
        .unwrap();
    let y = tag
        .split_once("y=\"")
        .unwrap()
        .1
        .split_once("\"")
        .unwrap()
        .0
        .parse()
        .unwrap();

    let mut split = data.split(',');

    let mut chunk = vec![0; 16 * 16];
    for item in &mut chunk {
        let a = split.next().unwrap().trim();
        *item = a.parse().unwrap()
    }
    Chunk { x, y, tiles: chunk }
}
