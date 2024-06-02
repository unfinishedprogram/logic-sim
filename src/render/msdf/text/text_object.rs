use glam::{Vec2, Vec4};

use crate::render::frame::Frame;

use super::MsdfFontReference;

// Defines some text to render
pub struct TextObject {
    pub content: String,
    pub position: Vec2,
    pub scale: f32,
}

impl TextObject {
    pub fn draw(&self, frame: &mut Frame, font: &MsdfFontReference) {
        let mut offset = Vec2::ZERO;

        for c in self.content.chars() {
            if c == '\n' {
                offset.x = 0.0;
                offset.y += 1.2;
                continue;
            }
            if let Some(sprite) = font.get(c) {
                frame
                    .draw_sprite(sprite, self.position + offset * self.scale, self.scale)
                    .on_click(|it| it.color(Vec4::new(1.0, 0.0, 0.0, 1.0)));
            }

            offset.x += font.advance(c)
        }
    }
}
