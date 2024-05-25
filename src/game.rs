use glam::Vec2;

use crate::render::msdf::{
    sprite::sprite_sheet::SpriteInstance,
    sprite_renderer::SpriteRendererReference,
    text::{MsdfFontReference, TextObject},
};

pub struct GameState {
    pub text_object: TextObject,
    sprites: SpriteRendererReference,
    font: MsdfFontReference,
}

impl GameState {
    pub fn new(font: MsdfFontReference, sprites: SpriteRendererReference) -> Self {
        let text_object = TextObject {
            content: "".to_string(),
            position: Vec2::new(0.0, -1.0),
            scale: 1.0,
        };

        Self {
            text_object,
            font,
            sprites,
        }
    }

    pub fn get_sprite_instances(&self) -> Vec<SpriteInstance> {
        let mut sprites: Vec<SpriteInstance> = self.text_object.as_sprite_instances(&self.font);

        for (index, gate) in ["AND", "BUF", "OR", "XOR", "XNOR", "NOT"]
            .iter()
            .enumerate()
        {
            let sprite = self
                .sprites
                .get_sprite("gates", gate)
                .unwrap()
                .instantiate((index as f32, 1.0).into(), 1.0);

            sprites.push(sprite);
        }

        sprites
    }

    pub fn render() {}
}
