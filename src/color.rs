use glam::Vec4;

const fn rgb(r: u8, g: u8, b: u8) -> Vec4 {
    Vec4::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

pub const RED: Vec4 = rgb(238, 99, 82);
pub const GREEN: Vec4 = rgb(89, 205, 144);
pub const BLUE: Vec4 = rgb(63, 167, 214);
pub const YELLOW: Vec4 = rgb(250, 192, 94);
pub const WHITE: Vec4 = rgb(255, 229, 212);
