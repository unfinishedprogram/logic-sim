use glam::{vec2, Vec2, Vec4};

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
        let mut x_offset: f32 = 0.0;

        for c in self.content.chars() {
            if let Some(sprite) = font.get(c) {
                frame
                    .draw_sprite(
                        sprite,
                        self.position + vec2(x_offset * self.scale, 0.0),
                        self.scale,
                    )
                    .on_click(|it| it.color(Vec4::new(1.0, 0.0, 0.0, 1.0)));
            }

            x_offset += font.advance(c)
        }
    }
}
