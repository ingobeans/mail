use std::env::args;

use macroquad::{miniquad::window::screen_size, prelude::*, time};

use crate::{
    assets::*,
    entities::{get_entities, show_tooltip},
    player::*,
    utils::*,
};

mod assets;
mod entities;
mod player;
mod utils;

fn window_conf() -> Conf {
    Conf {
        window_title: "mail".to_string(),
        window_width: SCREEN_WIDTH as i32 * 3,
        window_height: SCREEN_HEIGHT as i32 * 3,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    if args().any(|f| f == "debug") {
        *IS_DEBUG.lock().unwrap() = true;
    }
    let assets = Assets::default();
    let mut pixel_camera = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut world = World::default();
    let mut entities = get_entities(&world);
    let mut player = Player::new();

    player.pos = Vec2::new(-6.0 * 8.0, 2.0 * 8.0);
    player.facing_right = false;

    let mut last = time::get_time();

    let mut gift_select_index = 0;

    loop {
        let (actual_screen_width, actual_screen_height) = screen_size();
        let scale_factor =
            (actual_screen_width / SCREEN_WIDTH).min(actual_screen_height / SCREEN_HEIGHT);
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_x = mouse_x / scale_factor;
        let mouse_y = mouse_y / scale_factor;

        // handle gift selection screen
        if player.tags.contains(&Tag::SelectingGift) && !player.tags.contains(&Tag::HasGift) {
            pixel_camera.target = Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
            set_camera(&pixel_camera);
            // draw background
            clear_background(Color::from_hex(0x422433));
            draw_texture(&assets.gift_selection_screen, 0.0, 0.0, WHITE);

            // draw text
            let text = "select a gift";
            let padding = 2.0;
            let margin = 2.0;
            let width = text.len() as f32 * 4.0 + padding * 2.0;
            let height = 5.0 + padding * 2.0;
            let x = (SCREEN_WIDTH - width) / 2.0;
            let y = margin;
            draw_rectangle(x, y, width, height, WHITE);
            assets.draw_text(text, x + padding, y + padding);

            // draw buttons
            if draw_button(
                &assets.arrow,
                &assets.arrow_hovered,
                200.0,
                60.0,
                mouse_x,
                mouse_y,
                false,
            ) || is_key_pressed(KeyCode::D)
                || is_key_pressed(KeyCode::Right)
            {
                gift_select_index += 1;
            }

            if draw_button(
                &assets.arrow,
                &assets.arrow_hovered,
                36.0,
                60.0,
                mouse_x,
                mouse_y,
                true,
            ) || is_key_pressed(KeyCode::A)
                || is_key_pressed(KeyCode::Left)
            {
                if gift_select_index == 0 {
                    gift_select_index = assets.gift_sprites.total_length - 1;
                } else {
                    gift_select_index -= 1;
                }
            }

            // draw gift selection
            let gift_texture = assets.gift_sprites.get_at_time(gift_select_index);
            draw_texture(
                gift_texture,
                (SCREEN_WIDTH - gift_texture.width()) / 2.0,
                (SCREEN_HEIGHT - gift_texture.height()) / 2.0,
                WHITE,
            );

            // temporarily move player because the `show_tooltip` function is relative to player pos
            // but since this menu has a fixed pos camera, this needs to be disabled
            let mut old = Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
            (player.pos, old) = (old, player.pos);
            show_tooltip("e: select this gift", Tag::HasGift, &assets, &mut player);
            player.pos = old;
        } else {
            let now = time::get_time();

            if now - last > 1.0 / 60.0 {
                last = now;
                player.update(&world);
            }
            pixel_camera.target = player.pos.floor();
            set_camera(&pixel_camera);

            clear_background(Color::from_hex(0x249fde));

            for chunk in world.background.iter() {
                chunk.draw(&assets);
            }
            for chunk in world.collision.iter() {
                chunk.draw(&assets);
            }
            for chunk in world.details.iter() {
                chunk.draw(&assets);
            }
            for chunk in world.one_way_collision.iter() {
                chunk.draw(&assets);
            }

            for entity in entities.iter_mut() {
                entity.draw(&mut player, &assets);
            }

            if player.tags.contains(&Tag::TonyHasOpenedDoor) {
                world.set_collision_tile(79, 1, 0);
                world.set_collision_tile(79, 2, 0);
            }

            player.draw(&assets);
        }

        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &pixel_camera.render_target.as_ref().unwrap().texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    SCREEN_WIDTH * scale_factor,
                    SCREEN_HEIGHT * scale_factor,
                )),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
