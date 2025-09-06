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

pub struct Player {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub anim_frame: u32,
    pub facing_right: bool,
    idle_animation: Animation,
    walk_animation: Animation,
}
impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
            anim_frame: 0,
            facing_right: true,
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
            self.facing_right = false;
        }
        if is_key_down(KeyCode::D) {
            forces.x += 1.0;
            self.facing_right = true;
        }

        if is_key_down(KeyCode::Space) && self.velocity.y == 0.0 {
            forces.y -= 6.0;
        }

        forces.x -= self.velocity.x * GROUND_FRICTION;

        self.velocity += forces;

        let mut new = self.pos + self.velocity;

        let tile_x = self.pos.x / 8.0;
        let tile_y = self.pos.y / 8.0;

        let tiles_y = [
            (tile_x.floor(), (new.y / 8.0).ceil()),
            (tile_x.ceil(), (new.y / 8.0).ceil()),
            (tile_x.floor(), (new.y / 8.0).floor()),
            (tile_x.ceil(), (new.y / 8.0).floor()),
        ];

        let mut chunks: Vec<&Chunk> = Vec::new();
        let mut one_way_chunks: Vec<&Chunk> = Vec::new();
        for tile in tiles_y.iter() {
            let cx = ((tile.0 as f32 / 16.0).floor() * 16.0) as i16;
            let cy = ((tile.1 as f32 / 16.0).floor() * 16.0) as i16;
            if !chunks.iter().any(|f| f.x == cx && f.y == cy) {
                if let Some(chunk) = world.get_collision_chunk(cx, cy) {
                    chunks.push(chunk);
                }
            }
            if !one_way_chunks.iter().any(|f| f.x == cx && f.y == cy) {
                if let Some(chunk) = world.get_one_way_collision_chunk(cx, cy) {
                    one_way_chunks.push(chunk);
                }
            }
        }

        for (tx, ty) in tiles_y {
            draw_rectangle(tx.floor() * 8.0, ty.floor() * 8.0, 8.0, 8.0, RED);
            let tile = get_tile(&chunks, tx as i16, ty as i16);
            if tile != 0 {
                let c = if self.velocity.y < 0.0 {
                    tile_y.floor() * 8.0
                } else {
                    tile_y.ceil() * 8.0
                };
                new.y = c;
                self.velocity.y = 0.0;
                break;
            }
            if self.velocity.y > 0.0 {
                if get_tile(&one_way_chunks, tx as i16, ty as i16) != 0 {
                    new.y = tile_y.ceil() * 8.0;
                    self.velocity.y = 0.0;
                    break;
                }
            }
        }
        let tiles_x = [
            ((new.x / 8.0).floor(), (new.y / 8.0).ceil()),
            ((new.x / 8.0).ceil(), (new.y / 8.0).ceil()),
            ((new.x / 8.0).ceil(), (new.y / 8.0).floor()),
            ((new.x / 8.0).floor(), (new.y / 8.0).floor()),
        ];
        for (tx, ty) in tiles_x {
            //draw_rectangle(tx.floor() * 8.0, ty.floor() * 8.0, 8.0, 8.0, RED);
            let tile = get_tile(&chunks, tx as i16, ty as i16);
            if tile != 0 {
                let c = if self.velocity.x < 0.0 {
                    tile_x.floor() * 8.0
                } else {
                    tile_x.ceil() * 8.0
                };
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
    }
}
