#[macro_export]
macro_rules! rgba {
    ( $r:expr, $g:expr, $b:expr ) => {
        { macroquad::color::Color::new($r as f32 / 255f32, $g as f32 / 255f32, $b as f32 / 255f32, 1.0) }
    };
    ( $r:expr, $g:expr, $b:expr, $a:expr ) => {
        { macroquad::color::Color::new($r as f32 / 255f32, $g as f32 / 255f32, $b as f32 / 255f32, $a as f32 / 255f32) }
    };
}


#[macro_export] macro_rules! rgba_raw {
    ( $r:expr, $g:expr, $b:expr ) => {
        { ($r as f32 / 255f32, $g as f32 / 255f32, $b as f32 / 255f32, 1.0) }
    };
    ( $r:expr, $g:expr, $b:expr, $a:expr ) => {
        { ($r as f32 / 255f32, $g as f32 / 255f32, $b as f32 / 255f32, $a as f32 / 255f32) }
    };
}

/// Does absolutely nothing.
#[macro_export] macro_rules! pass {
    () => {{}};
}

pub use {rgba, rgba_raw, pass};