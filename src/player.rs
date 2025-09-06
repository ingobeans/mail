use macroquad::prelude::*;

use crate::{assets::*, utils::*};

fn get_tile(chunks: &[&Chunk], x: i16, y: i16) -> u8 {
    let cx = ((x as f32 / 16.0).floor() * 16.0) as i16;
    let cy = ((y as f32 / 16.0).floor() * 16.0) as i16;
    let Some(chunk) = chunks.iter().find(|f| f.x == cx && f.y == cy) else {
        return 0;
    };
    let local_x = x - chunk.x;
    let local_y = y - chunk.y;
    chunk.tile_at(local_x as _, local_y as _).unwrap_or(0)
}

pub struct Player {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub anim_frame: u32,
    idle_animation: Animation,
    walk_animation: Animation,
}
impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
            anim_frame: 0,
            idle_animation: Animation::from_file(include_bytes!(
                "../assets/entities/player/idle.ase"
            )),
            walk_animation: Animation::from_file(include_bytes!(
                "../assets/entities/player/walk.ase"
            )),
        }
    }
    pub fn update(&mut self, world: &World) {
        self.anim_frame += 1000 / 60;

        let mut forces = Vec2::ZERO;

        forces.y += GRAVITY;

        forces = forces.clamp_length_max(8.0);

        if is_key_down(KeyCode::A) {
            forces.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            forces.x += 1.0;
        }

        if is_key_down(KeyCode::Space) && self.velocity.y == 0.0 {
            forces.y -= 6.0;
        }

        forces.x -= self.velocity.x * GROUND_FRICTION;

        self.velocity += forces;

        if self.velocity.x.abs() < 0.3 {
            self.velocity.x = 0.0;
        }

        let mut new = self.pos + self.velocity;

        let tile_x = self.pos.x / 8.0;
        let tile_y = self.pos.y / 8.0;

        let tiles = [
            ((new.x / 8.0).floor(), tile_y.floor(), true),
            ((new.x / 8.0).ceil(), tile_y.ceil(), true),
            (tile_x.floor(), (new.y / 8.0).floor(), false),
            (tile_x.floor(), (new.y / 8.0).ceil(), false),
            (tile_x.ceil(), (new.y / 8.0).floor(), false),
        ];

        let mut chunks: Vec<&Chunk> = Vec::new();
        for tile in tiles.iter() {
            let cx = ((tile.0 as f32 / 16.0).floor() * 16.0) as i16;
            let cy = ((tile.1 as f32 / 16.0).floor() * 16.0) as i16;
            if !chunks.iter().any(|f| f.x == cx && f.y == cy) {
                if let Some(chunk) = world.get_collision_chunk(cx, cy) {
                    chunks.push(chunk);
                }
            }
        }

        for (tx, ty, x_axis) in tiles {
            let tile = get_tile(&chunks, tx as i16, ty as i16);
            if tile != 0 {
                if x_axis {
                    let c = if self.velocity.x < 0.0 {
                        tile_x.floor() * 8.0
                    } else {
                        tile_x.ceil() * 8.0
                    };
                    new.x = c;
                    self.velocity.x = 0.0;
                } else {
                    let c = if self.velocity.y < 0.0 {
                        tile_y.floor() * 8.0
                    } else {
                        tile_y.ceil() * 8.0
                    };
                    new.y = c;
                    self.velocity.y = 0.0;
                }
            }
        }

        self.pos = new;
    }
    pub fn draw(&self, _assets: &Assets) {
        let animation = if self.velocity.length() != 0.0 {
            &self.walk_animation
        } else {
            &self.idle_animation
        };
        draw_texture(
            animation.get_at_time(self.anim_frame),
            self.pos.floor().x,
            self.pos.floor().y,
            WHITE,
        );
    }
}
