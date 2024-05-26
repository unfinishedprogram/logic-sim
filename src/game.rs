pub mod game_loop;
pub mod input;
use glam::Vec2;

use crate::{
    logic::{circuit::Circuit, hit_test::HitTestResult},
    render::{
        camera::Camera,
        line::LineGeometry,
        msdf::{
            sprite::sprite_sheet::SpriteInstance,
            sprite_renderer::SpriteRendererReference,
            text::{MsdfFontReference, TextObject},
        },
    },
};

pub struct GameState {
    pub text_object: TextObject,
    pub camera: Camera,
    sprites: SpriteRendererReference,
    font: MsdfFontReference,
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

    pub fn get_sprite_instances(&self) -> Vec<SpriteInstance> {
        let mut sprites: Vec<SpriteInstance> = self.text_object.as_sprite_instances(&self.font);
        sprites.extend(self.circuit.sprite_instances(&self.sprites));
        sprites
    }

    pub fn get_line_instances(&self) -> Vec<LineGeometry> {
        self.circuit
            .connection_instances()
            .into_iter()
            .flat_map(|bezier| bezier.as_line_geometries(10, 0.05))
            .collect()
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
