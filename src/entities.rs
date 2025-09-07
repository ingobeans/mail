use std::sync::atomic::{AtomicU8, Ordering};

use crate::{
    assets::{Animation, Assets, World},
    player::{Player, Tag},
};
use macroquad::prelude::*;

pub enum DrawType {
    None,
    Animation(Animation),
    TextBubble(String),
}

pub struct Entity {
    pub pos: Vec2,
    pub draw_condition: &'static dyn Fn(&Entity, &mut Player) -> bool,
    pub draw_type: DrawType,
    pub lifetime: u32,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            pos: Vec2::ZERO,
            draw_condition: &|_, _| true,
            draw_type: DrawType::None,
            lifetime: 0,
        }
    }
}

impl Entity {
    pub fn draw(&mut self, player: &mut Player, assets: &Assets) {
        if (self.draw_condition)(self, player) {
            match &self.draw_type {
                DrawType::None => {}
                DrawType::Animation(animation) => {
                    let texture = animation.get_at_time(self.lifetime);
                    draw_texture(
                        texture,
                        self.pos.x - texture.width() / 2.0,
                        self.pos.y - texture.height() / 2.0,
                        WHITE,
                    );
                }
                DrawType::TextBubble(text) => {
                    let lines = text.lines();
                    let mut width = 0;
                    for line in lines.clone() {
                        let len = line.trim().len();
                        if len > width {
                            width = len;
                        }
                    }
                    let vertical_offset = 16.0;
                    let padding = 4.0;
                    let height = lines.clone().count() as f32 * 5.0 + padding * 2.0;
                    let width = width as f32 * 4.0 + padding * 2.0;
                    draw_rectangle(
                        self.pos.x,
                        self.pos.y - vertical_offset,
                        width,
                        height,
                        WHITE,
                    );
                    assets.draw_text(
                        &text,
                        self.pos.x + padding,
                        self.pos.y + padding - vertical_offset,
                    );
                }
            }
            self.lifetime += 1000 / 60;
        }
    }
}

pub fn get_entities(world: &World) -> Vec<Entity> {
    vec![
        Entity {
            pos: world.get_interactable_spawn(64).unwrap(),
            draw_condition: &|this, player| {
                if player.tags.contains(&Tag::HasInteractedWithHenry) {
                    false
                } else {
                    if player.pos.distance(this.pos) <= 32.0 {
                        player.tags.push(Tag::HasInteractedWithHenry);
                    }
                    true
                }
            },
            draw_type: DrawType::Animation(Animation::from_file(include_bytes!(
                "../assets/entities/poi.ase"
            ))),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(64).unwrap(),
            draw_condition: &|this, player| player.pos.distance(this.pos) <= 32.0,
            draw_type: DrawType::TextBubble(String::from(
                "hi!
                please go to the town
                and deliver my mail",
            )),
            ..Default::default()
        },
    ]
}
