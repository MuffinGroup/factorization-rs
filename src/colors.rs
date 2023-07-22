#![allow(dead_code)]
pub const BLUE: (f32, f32, f32, f32) = (0.0, 0.0, 1.0, 1.0);
pub const RED: (f32, f32, f32, f32) = (1.0, 0.0, 0.0, 1.0);
pub const GREEN: (f32, f32, f32, f32) = (0.0, 1.0, 1.0, 1.0);
pub const YELLOW: (f32, f32, f32, f32) = (1.0, 1.0, 0.0, 1.0);
pub const BLACK: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0);
pub const WHITE: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

pub fn rgb(r: i32, g: i32, b: i32) -> (f32, f32, f32, f32) {
    (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}