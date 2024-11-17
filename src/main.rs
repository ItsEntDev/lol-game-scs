mod config;
mod common;

use macroquad::prelude::*;
use ::rand;
use config::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    // Set up state
    let mut text_timer = 0.5;
    let mut text = String::new();
    let mut x = screen_width() / 2.0 - 32.0;
    let mut y = screen_height() / 2.0 - 32.0;
    
    loop {
        // Calculate timings
        let delta = get_frame_time();
        text_timer += delta;

        // Modify state
        if text_timer >= 0.5 {
            text_timer = 0.0;
            text = format!("{}", rand::random::<u32>());
        }

        // Handle input
        if is_key_down(KeyCode::W) {
            y -= config::SQUARE_SPEED * delta;
        }
        if is_key_down(KeyCode::S) {
            y += config::SQUARE_SPEED * delta;
        }
        if is_key_down(KeyCode::A) {
            x -= config::SQUARE_SPEED * delta;
        }
        if is_key_down(KeyCode::D) {
            x += config::SQUARE_SPEED * delta;
        }

        // Do rendering
        draw_text(
            &text,
            6.0,
            32.0,
            48.0,
            config::DEFAULT_COLOUR,
        );

        draw_rectangle(
            x,
            y,
            64.0,
            64.0,
            config::DEFAULT_COLOUR,
        );

        // Wait until next frame
        next_frame().await
    }
}
