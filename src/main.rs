use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::{assets::*, utils::*};

mod assets;
mod utils;

#[macroquad::main("mail")]
async fn main() {
    let assets = Assets::default();
    let pixel_camera = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
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
