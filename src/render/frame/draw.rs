use glam::Vec2;

use crate::render::{
    line::LineGeometry,
    msdf::sprite::sprite_sheet::{Sprite, SpriteInstance},
};

use super::Frame;

impl Frame {
    #[inline]
    pub fn draw_sprite(&mut self, sprite: &Sprite, position: Vec2, scale: f32) {
        self.draw_sprite_instance(sprite.instantiate(position, scale));
    }

    #[inline]
    pub fn draw_sprite_instance(&mut self, sprite: SpriteInstance) {
        self.sprites.push(sprite);
    }

    #[inline]
    pub fn draw_line(&mut self, line: LineGeometry) {
        self.lines.push(line);
    }
}
