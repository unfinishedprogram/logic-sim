use glam::Vec2;

use crate::render::{geometry::TexturedQuad, vertex::VertexUV};

use super::manifest::Manifest;

// Defines some text to render
pub struct TextObject {
    pub content: String,
    pub position: Vec2,
    pub scale: f32,
}

impl TextObject {
    pub fn into_textured_quads(&self, manifest: &Manifest) -> Vec<TexturedQuad> {
        let mut quads = Vec::new();

        let mut x_offset: f32 = 0.0;
        let y_offset: i32 = 0;

        for c in self.content.chars() {
            let glyph = manifest.get_glyph(c);
            if let Some(bounds) = glyph.atlas_bounds {
                let uvs = manifest.uvs_of(&bounds);

                let bounds = &glyph.plane_bounds.unwrap_or(bounds);
                let (mut p1, mut p2) = bounds.into();

                p1 += Vec2::new(x_offset, 0.0);
                p2 += Vec2::new(x_offset, 0.0);

                p1 *= self.scale;
                p2 *= self.scale;

                p1 += self.position;
                p2 += self.position;

                let p1 = VertexUV(p1, uvs.0);

                let p2 = VertexUV(p2, uvs.1);

                quads.push(TexturedQuad::new(p1, p2));
            } else {
                quads.push(TexturedQuad::new(
                    VertexUV(Vec2::ZERO, Vec2::ZERO),
                    VertexUV(Vec2::ZERO, Vec2::ZERO),
                ));
            }

            x_offset += glyph.advance;
        }

        quads
    }
}
