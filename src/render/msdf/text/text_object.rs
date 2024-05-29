use glam::{vec2, Vec2};

use crate::render::msdf::sprite::sprite_sheet::SpriteInstance;

use super::MsdfFontReference;

// Defines some text to render
pub struct TextObject {
    pub content: String,
    pub position: Vec2,
    pub scale: f32,
}

impl TextObject {
    pub fn as_sprite_instances(&self, font: &MsdfFontReference) -> Vec<SpriteInstance> {
        let mut instances = Vec::new();
        let mut x_offset: f32 = 0.0;

        for c in self.content.chars() {
            if let Some(sprite) = font.sprite_lookup.get(&c.to_string()) {
                let instance = sprite
                    .instantiate(self.position + vec2(x_offset * self.scale, 0.0), self.scale);

                instances.push(instance);
            }

            x_offset += font.manifest.get_glyph(c).advance;
        }

        instances
    }
}
