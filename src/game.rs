use glam::Vec2;

use crate::{
    logic::circuit::Circuit,
    render::{
        camera::Camera,
        line::{cubic_bezier::CubicBezier, LineDescriptor, LineGeometry},
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
            circuit: Circuit::default(),
        }
    }

    pub fn get_sprite_instances(&self) -> Vec<SpriteInstance> {
        let mut sprites: Vec<SpriteInstance> = self.text_object.as_sprite_instances(&self.font);
        sprites.extend(self.circuit.sprite_instances(&self.sprites));
        sprites
    }

    pub fn get_line_instances(&self) -> Vec<LineGeometry> {
        let mut geo: Vec<LineGeometry> = vec![];

        geo.extend(
            CubicBezier::between_points(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 0.5))
                .as_line_geometries(10, 0.1),
        );

        geo.extend(
            CubicBezier::between_points(Vec2::new(-1.0, 3.0), Vec2::new(1.0, 3.0))
                .as_line_geometries(10, 0.1),
        );

        geo.extend(
            CubicBezier::between_points(Vec2::new(-1.0, -1.0), Vec2::new(-1.0, 3.0))
                .as_line_geometries(3, 0.1),
        );

        geo
    }
}

impl GameState {
    pub fn on_click(&mut self, position: Vec2) {
        let gates = [
            crate::logic::gate::Gate::And,
            crate::logic::gate::Gate::Or,
            crate::logic::gate::Gate::Not,
            crate::logic::gate::Gate::Buf,
            crate::logic::gate::Gate::Xor,
            crate::logic::gate::Gate::Nand,
            crate::logic::gate::Gate::Nor,
            crate::logic::gate::Gate::Xnor,
        ];

        for (i, gate) in gates.into_iter().enumerate() {
            self.circuit
                .add_gate(gate, position + Vec2::new(0.0, i as f32));
        }
    }
}
