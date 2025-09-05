use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::{assets::*, utils::*};

mod assets;
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
    let assets = Assets::default();
    let mut pixel_camera = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
    let world = World::default();

    loop {
        let (actual_screen_width, actual_screen_height) = screen_size();
        let scale_factor =
            (actual_screen_width / SCREEN_WIDTH).min(actual_screen_height / SCREEN_HEIGHT);
        set_camera(&pixel_camera);
        clear_background(WHITE);
        for chunk in world.collision.iter() {
            chunk.draw(&assets);
        }
        for chunk in world.details.iter() {
            chunk.draw(&assets);
        }

        if is_key_down(KeyCode::A) {
            pixel_camera.target.x -= 5.0;
        }
        if is_key_down(KeyCode::D) {
            pixel_camera.target.x += 5.0;
        }
        if is_key_down(KeyCode::W) {
            pixel_camera.target.y -= 5.0;
        }
        if is_key_down(KeyCode::S) {
            pixel_camera.target.y += 5.0;
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
