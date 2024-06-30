use glam::Vec2;

use crate::render::frame::RenderQueue;

use super::MsdfFontReference;

// Defines some text to render
pub struct TextObject {
    pub content: String,
    pub position: Vec2,
    pub scale: f32,
}

impl TextObject {
    pub fn draw(&self, render_queue: &mut RenderQueue, font: &MsdfFontReference) {
        let line_height = 1.2;
        let mut offset = Vec2::new(0.0, line_height);

        for c in self.content.chars() {
            if c == '\n' {
                offset.x = 0.0;
                offset.y += line_height;
                continue;
            }
            if let Some(sprite) = font.get(c) {
                render_queue.enqueue_sprite(
                    sprite.instantiate(self.position + offset * self.scale, self.scale),
                );
            }

            offset.x += font.advance(c)
        }
    }
}
