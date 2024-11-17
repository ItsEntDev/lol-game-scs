use macroquad::color::Color;
use macroquad::prelude::{screen_height, screen_width, Conf};
use crate::common;

pub const RESIZE_WINDOW: bool = true;
pub const FULLSCREEN: bool = false;
pub const WIDTH_WINDOW_FALLBACK: i32 = 1280;
pub const HEIGHT_WINDOW_FALLBACK: i32 = 720;
pub const NAME_WINDOW: &'static str = "Game Test";

pub const DEFAULT_COLOUR: Color = common::rgba!(51, 0, 175);
pub const SQUARE_SPEED: f32 = 128.0;

pub fn window_conf() -> Conf {
    if let Some(a) = std::env::args().nth(1) {
        if a == "-fallback" {
            fallback_conf()
        } else {
            default_conf()
        }
    } else {
        default_conf()
    }
}

fn default_conf() -> Conf {
    Conf {
        window_title: NAME_WINDOW.to_string(),
        fullscreen: FULLSCREEN,
        window_resizable: RESIZE_WINDOW,
        window_width: screen_width() as i32 / 2,
        window_height: screen_height() as i32 / 2,
        ..Default::default()
    }
}

fn fallback_conf() -> Conf {
    Conf {
        window_title: NAME_WINDOW.to_string(),
        fullscreen: FULLSCREEN,
        window_resizable: !RESIZE_WINDOW,
        window_width: WIDTH_WINDOW_FALLBACK,
        window_height: HEIGHT_WINDOW_FALLBACK,
        ..Default::default()
    }
}