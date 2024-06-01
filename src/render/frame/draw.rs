use glam::{Vec2, Vec4};

use crate::render::{
    line::LineGeometry,
    msdf::{sprite::sprite_sheet::SpriteInstance, sprite_renderer::SpriteHandle},
};

use super::Frame;

impl Frame {
    #[inline]
    pub fn draw_sprite(&mut self, sprite_handle: SpriteHandle, position: Vec2, scale: f32) {
        self.draw_sprite_instance(SpriteInstance {
            sprite_handle,
            position,
            scale,
            color: Vec4::splat(1.0),
        });
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
