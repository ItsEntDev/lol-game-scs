mod config;
mod common;

use macroquad::prelude::*;
use ::rand;
use config::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    let mut text_timer = 0.5;
    let mut text = String::new();
    

    
    loop {
        // Calculate timings
        let delta = get_frame_time();
        text_timer += delta;
        
        // Modify state
        if text_timer >= 0.5 {
            text_timer = 0.0;
            text = format!("{}", rand::random::<u32>());
        }
        
        // Do rendering
        draw_text(
            &text, 
            config::WIDTH_WINDOW_FALLBACK as f32 / 2.0, 
            config::HEIGHT_WINDOW_FALLBACK as f32 / 2.0, 
            48.0, 
            config::DEFAULT_COLOUR,
        );
        
        next_frame().await
    }
}
