use crate::render::{line::LineGeometry, msdf::sprite::sprite_sheet::SpriteInstance};

use super::Frame;

impl Frame {
    pub fn draw_sprite_instance(&mut self, sprite: SpriteInstance) {
        self.sprites.push(sprite);
    }
    pub fn draw_line(&mut self, line: LineGeometry) {
        self.lines.push(line);
    }
}
