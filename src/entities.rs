use crate::{
    assets::{Animation, Assets, World},
    player::{Player, Tag},
    utils::*,
};
use macroquad::prelude::*;

pub enum DrawType {
    None,
    Animation(Animation),
    TextBubble(String),
}

pub struct Entity {
    pub pos: Vec2,
    pub draw_condition: &'static dyn Fn(&mut Entity, &mut Player, &Assets) -> bool,
    pub draw_type: DrawType,
    pub anim_frame: u32,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            pos: Vec2::ZERO,
            draw_condition: &|_, _, _| true,
            draw_type: DrawType::None,
            anim_frame: 0,
        }
    }
}

impl Entity {
    pub fn draw(&mut self, player: &mut Player, assets: &Assets) {
        if (self.draw_condition)(self, player, assets) {
            match &self.draw_type {
                DrawType::None => {}
                DrawType::Animation(animation) => {
                    let texture = animation.get_at_time(self.anim_frame);
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
                        text,
                        self.pos.x + padding,
                        self.pos.y + padding - vertical_offset,
                    );
                }
            }
            self.anim_frame += 1000 / 60;
        }
    }
}

pub fn show_tooltip(text: &str, grants_tag: Tag, assets: &Assets, player: &mut Player) {
    let padding = 2.0;
    let margin = 2.0;

    let width = text.len() as f32 * 4.0 + padding * 2.0;
    let height = 5.0 + padding * 2.0;
    let x = (player.camera_pos.x - width / 2.0 + 4.0).floor();
    let y = (player.camera_pos.y - height - margin + SCREEN_HEIGHT / 2.0).floor();
    draw_rectangle(x, y, width, height, Color::from_hex(0x3b1725));
    draw_rectangle(
        x + 1.0,
        y + 1.0,
        width - 2.0,
        height - 2.0,
        Color::from_hex(0xfffc40),
    );
    assets.draw_text(text, x + padding, y + padding);
    if is_key_pressed(KeyCode::E) {
        player.tags.push(grants_tag);
    }
}

pub fn get_entities(world: &World) -> Vec<Entity> {
    vec![
        Entity {
            pos: world.get_interactable_spawn(64).unwrap(),
            draw_condition: &|this, player, _| {
                (!player.tags.contains(&Tag::HasMail)
                    || player.tags.contains(&Tag::HenryHasOfferedCarrot)
                    || player.tags.contains(&Tag::MailHasBeenSent))
                    && player.pos.distance(this.pos) > 32.0
            },
            draw_type: DrawType::Animation(Animation::from_file(include_bytes!(
                "../assets/entities/poi.ase"
            ))),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(64).unwrap(),
            draw_condition: &|this, player, assets| {
                if !player.tags.contains(&Tag::HasMail) {
                    if player.pos.distance(this.pos) <= 32.0 {
                        if player.pos.distance(this.pos) <= 32.0 {
                            show_tooltip("e: take mail", Tag::HasMail, assets, player);
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            draw_type: DrawType::TextBubble(String::from(
                "hi!
                please go to the town
                and post my mail",
            )),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(64).unwrap(),
            draw_condition: &|this, player, _| {
                player.pos.distance(this.pos) <= 32.0
                    && player.tags.contains(&Tag::HasMail)
                    && !player.tags.contains(&Tag::MailHasBeenSent)
            },
            draw_type: DrawType::TextBubble(String::from(
                "thanks! return when
                you have posted it",
            )),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(64).unwrap() + Vec2::new(-4.0, 12.0),
            draw_condition: &|this, player, _| {
                if !player.tags.contains(&Tag::HasReturnedToHenry) {
                    player.tags.push(Tag::HasReturnedToHenry);
                }
                if this.anim_frame >= 3750 {
                    player.tags.push(Tag::HenryHasOfferedCarrot);
                    this.anim_frame = 3700;
                }
                (player.pos.distance(this.pos) <= 32.0 || this.anim_frame > 0)
                    && player.tags.contains(&Tag::MailHasBeenSent)
            },
            draw_type: DrawType::Animation(Animation::from_file(include_bytes!(
                "../assets/entities/henry_get_reward.ase"
            ))),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(64).unwrap(),
            draw_condition: &|this, player, assets| {
                if player.tags.contains(&Tag::HenryHasOfferedCarrot) {
                    if player.pos.distance(this.pos) <= 32.0 {
                        if player.pos.distance(this.pos) <= 32.0 {
                            show_tooltip("e: accept carrot", Tag::HasCarrot, assets, player);
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            draw_type: DrawType::TextBubble(String::from(
                "take this carrot
                as a reward!",
            )),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(128).unwrap(),
            draw_condition: &|this, player, _| {
                player.pos.distance(this.pos) > 32.0
                    && (!player.tags.contains(&Tag::HasBirdFood)
                        || (player.tags.contains(&Tag::HasFedBird)
                            && !player.tags.contains(&Tag::TonyHasOpenedDoor))
                        || player.tags.contains(&Tag::HasBeeninGiftStore)
                            && !player.tags.contains(&Tag::HasMilk))
            },
            draw_type: DrawType::Animation(Animation::from_file(include_bytes!(
                "../assets/entities/poi.ase"
            ))),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(128).unwrap(),
            draw_condition: &|this, player, assets| {
                if !player.tags.contains(&Tag::HasBirdFood) {
                    if player.pos.distance(this.pos) <= 32.0 {
                        if player.pos.distance(this.pos) <= 32.0 {
                            show_tooltip("e: take bird food", Tag::HasBirdFood, assets, player);
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            draw_type: DrawType::TextBubble(String::from(
                "hi!
                feed the bird on my roof
                and i will let you pass 
                through here",
            )),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(128).unwrap(),
            draw_condition: &|this, player, _| {
                player.tags.contains(&Tag::HasBirdFood)
                    && !player.tags.contains(&Tag::HasFedBird)
                    && player.pos.distance(this.pos) <= 32.0
            },
            draw_type: DrawType::TextBubble(String::from(
                "return when youve fed
                the bird on my roof",
            )),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(128).unwrap(),
            draw_condition: &|this, player, _| {
                if !player.tags.contains(&Tag::HasMilk)
                    && player.tags.contains(&Tag::HasFedBird)
                    && player.pos.distance(this.pos) <= 32.0
                {
                    player.tags.push(Tag::TonyHasOpenedDoor);
                    true
                } else {
                    false
                }
            },
            draw_type: DrawType::TextBubble(String::from("thanks!")),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(612).unwrap(),
            draw_condition: &|_, player, _| !player.tags.contains(&Tag::HasFedBird),
            draw_type: DrawType::Animation(Animation::from_file(include_bytes!(
                "../assets/entities/bird.ase"
            ))),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(612).unwrap(),
            draw_condition: &|this, player, assets| {
                if player.tags.contains(&Tag::HasBirdFood)
                    && !player.tags.contains(&Tag::HasFedBird)
                    && player.pos.distance(this.pos) <= 8.0
                {
                    show_tooltip("e: feed bird", Tag::HasFedBird, assets, player);
                }
                false
            },
            draw_type: DrawType::None,
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(612).unwrap(),
            draw_condition: &|_, player, _| player.tags.contains(&Tag::HasFedBird),
            draw_type: DrawType::Animation(Animation::from_file(include_bytes!(
                "../assets/entities/bird_eating.ase"
            ))),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(288).unwrap(),
            draw_condition: &|this, player, _| {
                player.tags.contains(&Tag::TonyHasOpenedDoor)
                    && !player.tags.contains(&Tag::HasGivenGift)
                    && player.pos.distance(this.pos) > 32.0
            },
            draw_type: DrawType::Animation(Animation::from_file(include_bytes!(
                "../assets/entities/poi.ase"
            ))),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(288).unwrap(),
            draw_condition: &|this, player, assets| {
                if player.tags.contains(&Tag::HasMail) && !player.tags.contains(&Tag::HasGivenGift)
                {
                    if player.pos.distance(this.pos) <= 32.0 {
                        if player.tags.contains(&Tag::HasGift) {
                            show_tooltip("e: give gift", Tag::HasGivenGift, assets, player);
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            draw_type: DrawType::TextBubble(String::from(
                "today is my birthday.
                want to send mail?
                get me a gift!",
            )),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(288).unwrap() + Vec2::new(0.0, 20.0),
            draw_condition: &|this, player, _| {
                if this.anim_frame > 650 {
                    player.tags.push(Tag::MailHasBeenSent);
                }
                player.tags.contains(&Tag::HasGivenGift)
                    && !player.tags.contains(&Tag::MailHasBeenSent)
            },
            draw_type: DrawType::Animation(Animation::from_file(include_bytes!(
                "../assets/entities/birthday_happy.ase"
            ))),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(288).unwrap(),
            draw_condition: &|this, player, _| {
                if player.tags.contains(&Tag::MailHasBeenSent) {
                    player.pos.distance(this.pos) <= 32.0
                } else {
                    false
                }
            },
            draw_type: DrawType::TextBubble(String::from(
                "thanks for the gift!
                ive sent your mail now",
            )),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(384).unwrap(),
            draw_condition: &|this, player, _| {
                !player.tags.contains(&Tag::HasGift) && player.pos.distance(this.pos) > 32.0
            },
            draw_type: DrawType::Animation(Animation::from_file(include_bytes!(
                "../assets/entities/poi.ase"
            ))),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(384).unwrap(),
            draw_condition: &|this, player, _| {
                if !player.tags.contains(&Tag::HasMilk) {
                    if player.pos.distance(this.pos) <= 32.0 {
                        player.tags.push(Tag::HasBeeninGiftStore);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            draw_type: DrawType::TextBubble(String::from(
                "buy me some milk from
                tonys grocery and i will
                give you a gift to give",
            )),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(384).unwrap(),
            draw_condition: &|this, player, _| {
                player.tags.contains(&Tag::HasGift) && player.pos.distance(this.pos) <= 32.0
            },
            draw_type: DrawType::TextBubble(String::from(
                "pleasure doin business
                with you!",
            )),
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(384).unwrap(),
            draw_condition: &|this, player, assets| {
                if player.tags.contains(&Tag::HasMilk)
                    && !player.tags.contains(&Tag::HasGift)
                    && player.pos.distance(this.pos) <= 32.0
                {
                    show_tooltip("e: give milk", Tag::SelectingGift, assets, player);
                }
                false
            },
            draw_type: DrawType::None,
            ..Default::default()
        },
        Entity {
            pos: world.get_interactable_spawn(128).unwrap(),
            draw_condition: &|this, player, assets| {
                if player.tags.contains(&Tag::HasBeeninGiftStore)
                    && !player.tags.contains(&Tag::HasMilk)
                    && player.pos.distance(this.pos) <= 32.0
                {
                    show_tooltip("e: accept milk", Tag::HasMilk, assets, player);
                    true
                } else {
                    false
                }
            },
            draw_type: DrawType::TextBubble(String::from(
                "here! have some milk
                as thanks for feeding
                my bird",
            )),
            ..Default::default()
        },
    ]
}
