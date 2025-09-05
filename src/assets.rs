use asefile::AsepriteFile;
use image::EncodableLayout;
use macroquad::prelude::*;

use crate::utils::*;

pub struct Assets {
    pub tileset: Spritesheet,
}
impl Default for Assets {
    fn default() -> Self {
        Self {
            tileset: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/tileset.ase"), None),
                8.0,
            ),
        }
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

pub struct World {
    pub collision: Vec<Chunk>,
    pub details: Vec<Chunk>,
}
impl Default for World {
    fn default() -> Self {
        let xml = include_str!("../assets/tilemap/world.tmx");
        let collisions = get_layer(xml, "Collisions");
        let details = get_layer(xml, "Details");
        World {
            collision: get_all_chunks(collisions),
            details: get_all_chunks(details),
        }
    }
}

pub struct Chunk {
    x: i16,
    y: i16,
    tiles: Vec<u8>,
}
impl Chunk {
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
    let count = xml.lines().filter(|f| *f == "</chunk>").count();
    println!("chunks amt: {count}");
    let mut chunks = Vec::new();
    let mut xml = xml.to_string();
    loop {
        if let Some((current, remains)) = xml.split_once("</chunk>") {
            let new = parse_chunk(&current.clone());
            chunks.push(new);
            xml = remains.to_string();
        } else {
            break;
        }
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
    println!("{tag}");

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
