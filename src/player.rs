use macroquad::prelude::*;

use crate::{assets::*, utils::*};

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
    pub fn update(&mut self) {
        self.anim_frame += 1000 / 60;
        if is_key_down(KeyCode::A) {
            self.velocity.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            self.velocity.x += 1.0;
        }
        if is_key_down(KeyCode::W) {
            self.velocity.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            self.velocity.y += 1.0;
        }

        self.velocity = self
            .velocity
            .lerp(Vec2::ZERO, GROUND_DRAG)
            .clamp_length_max(MAX_VELOCITY);
        if self.velocity.length() < 0.3 {
            self.velocity = Vec2::ZERO;
        }
        self.pos += self.velocity;
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
