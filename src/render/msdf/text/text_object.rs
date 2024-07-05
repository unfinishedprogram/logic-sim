use glam::Vec2;

use crate::render::frame::RenderQueue;

use super::MsdfFontReference;

// Defines some text to render
pub struct TextObject {
    pub content: String,
    pub position: Vec2,
    pub scale: f32,
    pub centered: bool,
}

const LINE_HEIGHT: f32 = 1.2;

impl TextObject {
    pub fn draw(&self, render_queue: &mut RenderQueue, font: &MsdfFontReference) {
        let center_offset = if self.centered {
            -self.center_mono()
        } else {
            Vec2::ZERO
        };

        let mut offset = Vec2::Y * LINE_HEIGHT;

        for c in self.content.chars() {
            if c == '\n' {
                offset.x = 0.0;
                offset.y += LINE_HEIGHT;
                continue;
            }
            if let Some(sprite) = font.get(c) {
                render_queue.enqueue_sprite(sprite.instantiate(
                    self.position + offset * self.scale + center_offset,
                    self.scale,
                ));
            }

            offset.x += font.advance(c)
        }
    }

    pub fn center_mono(&self) -> Vec2 {
        // Positioning is done from the bottom left of a char
        let lines = self.content.split('\n');

        let mut max_width = 0.0;
        let mut height = 0.0;
        for line in lines {
            // Hardcoded for monospace
            let advance = 0.6;
            let line_width = line.chars().count() as f32 * advance;
            max_width = line_width.max(max_width);
            height += LINE_HEIGHT;
        }

        Vec2::new(max_width, height + LINE_HEIGHT / 2.0) * self.scale / 2.0
    }
}
