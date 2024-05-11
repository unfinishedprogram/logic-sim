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

        let mut x_offset: i32 = 0;
        let y_offset: i32 = 0;

        for c in self.content.chars() {
            let char_info = manifest.get_char(c);
            let uvs = manifest.uvs_of(char_info);

            let x1 = char_info.xoffset + x_offset;
            let y1 = char_info.yoffset + y_offset;

            let x2 = x1 + char_info.width as i32;
            let y2 = y1 + char_info.height as i32;

            let p1 = VertexUV(
                Vec2::new(x1 as f32, y1 as f32) * self.scale + self.position,
                uvs.0,
            );

            let p2 = VertexUV(
                Vec2::new(x2 as f32, y2 as f32) * self.scale + self.position,
                uvs.1,
            );

            quads.push(TexturedQuad::new(p1, p2));

            x_offset += char_info.xadvance as i32;
        }

        quads
    }
}
