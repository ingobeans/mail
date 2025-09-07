use macroquad::prelude::*;

use crate::{assets::*, utils::*};

fn get_tile(chunks: &[&Chunk], x: i16, y: i16) -> i16 {
    let cx = ((x as f32 / 16.0).floor() * 16.0) as i16;
    let cy = ((y as f32 / 16.0).floor() * 16.0) as i16;
    let Some(chunk) = chunks.iter().find(|f| f.x == cx && f.y == cy) else {
        return 0;
    };
    let local_x = x - chunk.x;
    let local_y = y - chunk.y;
    chunk.tile_at(local_x as _, local_y as _).unwrap_or(0)
}

fn ceil_g(a: f32) -> f32 {
    if a < 0.0 { a.floor() } else { a.ceil() }
}

#[derive(PartialEq)]
pub enum Tag {
    HasMail,
    HasBirdFood,
    HasFedBird,
    TonyHasOpenedDoor,
    HasGift,
    HasGivenGift,
    MailHasBeenSent,
    HasBeeninGiftStore,
    HasMilk,
}

pub struct Player {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub anim_frame: u32,
    pub facing_right: bool,
    pub on_ground: bool,
    pub debug_tiles: Vec<(f32, f32, f32, f32, Color)>,
    pub jump_frames: u8,
    pub tags: Vec<Tag>,
    idle_animation: Animation,
    walk_animation: Animation,
}
impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
            anim_frame: 0,
            jump_frames: 0,
            facing_right: true,
            on_ground: false,
            debug_tiles: Vec::new(),
            tags: Vec::new(),
            idle_animation: Animation::from_file(include_bytes!(
                "../assets/entities/player/idle.ase"
            )),
            walk_animation: Animation::from_file(include_bytes!(
                "../assets/entities/player/walk.ase"
            )),
        }
    }
    pub fn update(&mut self, world: &World) {
        self.debug_tiles = Vec::new();
        self.anim_frame += 1000 / 60;

        let noclip = is_key_down(KeyCode::LeftShift);

        let mut forces = Vec2::ZERO;

        if !noclip {
            forces.y += GRAVITY
        }

        forces = forces.clamp_length_max(8.0);

        if is_key_down(KeyCode::A) {
            forces.x -= 1.0;
            self.facing_right = false;
        }
        if is_key_down(KeyCode::D) {
            forces.x += 1.0;
            self.facing_right = true;
        }

        if self.on_ground {
            self.jump_frames = 0;
        }
        if is_key_down(KeyCode::Space)
            && (self.on_ground || (self.jump_frames > 0 && self.jump_frames < 5))
        {
            forces.y -= if self.jump_frames == 0 { 3.5 } else { 1.0 };
            self.jump_frames += 1;
        }

        if noclip {
            if is_key_down(KeyCode::W) {
                forces.y -= 1.0;
            }
            if is_key_down(KeyCode::S) {
                forces.y += 1.0;
            }
            self.velocity += forces * 2.0;
            self.velocity = self.velocity.lerp(Vec2::ZERO, GROUND_FRICTION);

            self.pos += self.velocity;
            return;
        }

        forces.x -= self.velocity.x
            * if self.on_ground {
                GROUND_FRICTION
            } else {
                AIR_DRAG
            };

        self.velocity += forces;

        let mut new = self.pos + self.velocity;

        let tile_x = self.pos.x / 8.0;
        let tile_y = self.pos.y / 8.0;

        let tiles_y = [
            (tile_x.trunc(), ceil_g(new.y / 8.0)),
            (ceil_g(tile_x), ceil_g(new.y / 8.0)),
            (tile_x.trunc(), (new.y / 8.0).trunc()),
            (ceil_g(tile_x), (new.y / 8.0).trunc()),
        ];

        let mut chunks: Vec<&Chunk> = Vec::new();
        let mut one_way_chunks: Vec<&Chunk> = Vec::new();

        for tile in tiles_y.iter() {
            let cx = ((tile.0 / 16.0).floor() * 16.0) as i16;
            let cy = ((tile.1 / 16.0).floor() * 16.0) as i16;
            if !chunks.iter().any(|f| f.x == cx && f.y == cy)
                && let Some(chunk) = world.get_collision_chunk(cx, cy)
            {
                chunks.push(chunk);
                self.debug_tiles.push((
                    cx as f32 * 8.0,
                    cy as f32 * 8.0,
                    16.0 * 8.0,
                    16.0 * 8.0,
                    Color::from_rgba(255, 0, 255, 125),
                ));
            }
            if !one_way_chunks.iter().any(|f| f.x == cx && f.y == cy)
                && let Some(chunk) = world.get_one_way_collision_chunk(cx, cy)
            {
                one_way_chunks.push(chunk);
            }
        }

        self.on_ground = false;
        for (tx, ty) in tiles_y {
            let tile = get_tile(&chunks, tx as i16, ty as i16);
            if tile != 0 {
                self.debug_tiles
                    .push((tx.trunc() * 8.0, ty.trunc() * 8.0, 8.0, 8.0, RED));
                let c = if self.velocity.y < 0.0 {
                    tile_y.floor() * 8.0
                } else {
                    self.on_ground = true;
                    tile_y.ceil() * 8.0
                };
                new.y = c;
                self.velocity.y = 0.0;
                break;
            }

            // handle single way platforms
            if self.velocity.y > 0.0
                && ty.trunc() > tile_y.trunc()
                && get_tile(&one_way_chunks, tx as i16, ty as i16) != 0
            {
                new.y = tile_y.ceil() * 8.0;
                self.debug_tiles
                    .push((tx.trunc() * 8.0, ty.trunc() * 8.0, 8.0, 8.0, YELLOW));
                self.velocity.y = 0.0;
                self.on_ground = true;
                break;
            }
        }
        self.debug_tiles
            .push((tile_x.floor() * 8.0, tile_y.floor() * 8.0, 1.0, 1.0, LIME));
        self.debug_tiles
            .push((tile_x.ceil() * 8.0, tile_y.floor() * 8.0, 1.0, 1.0, GREEN));
        self.debug_tiles
            .push((tile_x * 8.0, tile_y * 8.0, 1.0, 1.0, GOLD));
        let tiles_x = [
            ((new.x / 8.0).trunc(), ceil_g(new.y / 8.0)),
            (ceil_g(new.x / 8.0), ceil_g(new.y / 8.0)),
            (ceil_g(new.x / 8.0), (new.y / 8.0).trunc()),
            ((new.x / 8.0).trunc(), (new.y / 8.0).trunc()),
        ];

        for tile in tiles_x.iter() {
            let cx = ((tile.0 / 16.0).floor() * 16.0) as i16;
            let cy = ((tile.1 / 16.0).floor() * 16.0) as i16;
            if !chunks.iter().any(|f| f.x == cx && f.y == cy)
                && let Some(chunk) = world.get_collision_chunk(cx, cy)
            {
                chunks.push(chunk);
                self.debug_tiles.push((
                    cx as f32 * 8.0,
                    cy as f32 * 8.0,
                    16.0 * 8.0,
                    16.0 * 8.0,
                    Color::from_rgba(255, 0, 255, 125),
                ));
            }
            if !one_way_chunks.iter().any(|f| f.x == cx && f.y == cy)
                && let Some(chunk) = world.get_one_way_collision_chunk(cx, cy)
            {
                one_way_chunks.push(chunk);
            }
        }
        for (tx, ty) in tiles_x {
            self.debug_tiles.push((
                tx.trunc() * 8.0,
                ty.trunc() * 8.0,
                8.0,
                8.0,
                BEIGE.with_alpha(0.2),
            ));
            let tile = get_tile(&chunks, tx as i16, ty as i16);
            if tile != 0 {
                let c = if self.velocity.x < 0.0 {
                    tile_x.floor() * 8.0
                } else {
                    tile_x.ceil() * 8.0
                };
                self.debug_tiles
                    .push((tx.trunc() * 8.0, ty.trunc() * 8.0, 8.0, 8.0, BLUE));
                new.x = c;
                self.velocity.x = 0.0;
                break;
            }
        }

        if self.velocity.x.abs() <= 0.3 {
            self.velocity.x = 0.0;
        }
        self.velocity.x = self.velocity.x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        self.pos = new;
    }
    pub fn draw(&self, _assets: &Assets) {
        let animation = if self.velocity.length() != 0.0 {
            &self.walk_animation
        } else {
            &self.idle_animation
        };
        draw_texture_ex(
            animation.get_at_time(self.anim_frame),
            self.pos.floor().x,
            self.pos.floor().y,
            WHITE,
            DrawTextureParams {
                flip_x: !self.facing_right,
                ..Default::default()
            },
        );
        if *IS_DEBUG.lock().unwrap() {
            for (x, y, w, h, color) in self.debug_tiles.iter() {
                draw_rectangle(*x, *y, *w, *h, *color);
            }
        }
    }
}
