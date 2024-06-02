pub mod game_loop;
pub mod input;
use glam::Vec2;

use crate::{
    logic::{circuit::Circuit, hit_test::HitTestResult},
    render::{
        camera::Camera,
        msdf::{
            sprite_renderer::SpriteRendererReference,
            text::{MsdfFontReference, TextObject},
        },
    },
};

pub struct GameState {
    pub text_object: TextObject,
    pub camera: Camera,
    sprites: SpriteRendererReference,
    pub font: MsdfFontReference,
    circuit: Circuit,
}

impl GameState {
    pub fn new(font: MsdfFontReference, sprites: SpriteRendererReference) -> Self {
        let text_object = TextObject {
            content: "".to_string(),
            position: Vec2::new(0.0, -1.0),
            scale: 1.0,
        };

        Self {
            camera: Camera::new(),
            text_object,
            font,
            sprites,
            circuit: Circuit::test_circuit(),
        }
    }
}

impl GameState {
    pub fn on_click(&mut self, position: Vec2) {
        let hit = self.circuit.hit_test(position);

        if let HitTestResult::Element(idx) = hit {
            self.circuit.remove_gate(idx);
        }
    }
}
