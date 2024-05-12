use glam::{vec2, Vec2};

use crate::render::{geometry::TexturedQuad, vertex::VertexUV};

use super::MsdfFont;

// Defines some text to render
pub struct TextObject {
    pub content: String,
    pub position: Vec2,
    pub scale: f32,
}

impl TextObject {
    pub fn as_textured_quads(&self, font: &MsdfFont) -> Vec<TexturedQuad> {
        let mut quads = Vec::new();
        let mut x_offset: f32 = 0.0;

        for c in self.content.chars() {
            if let Some(sprite) = font.sprite_sheet.get_sprite(&c.to_string()) {
                let instance = sprite
                    .instantiate(self.position + vec2(x_offset * self.scale, 0.0), self.scale);

                quads.push(instance.into());
            } else {
                quads.push(TexturedQuad::new(
                    VertexUV(Vec2::ZERO, Vec2::ZERO),
                    VertexUV(Vec2::ZERO, Vec2::ZERO),
                ));
            }

            x_offset += font.manifest.get_glyph(c).advance;
        }

        quads
    }
}
