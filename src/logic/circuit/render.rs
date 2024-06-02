use crate::render::line::cubic_bezier::CubicBezier;
use crate::render::msdf::sprite::sprite_sheet::SpriteInstance;
use crate::render::msdf::sprite_renderer::SpriteRendererReference;

use super::super::gate::Gate;
use super::Circuit;

const GATE_SHEET: &str = "gates";

pub fn sprite_of(gate: &Gate) -> Option<&'static str> {
    match gate {
        Gate::Input(_) => None,
        Gate::And => Some("AND"),
        Gate::Or => Some("OR"),
        Gate::Not => Some("NOT"),
        Gate::Buf => Some("BUF"),
        Gate::Xor => Some("XOR"),
        Gate::Nand => Some("NAND"),
        Gate::Nor => Some("NOR"),
        Gate::Xnor => Some("XNOR"),
    }
}

impl Circuit {
    pub fn sprite_instances(&self, sheets: &SpriteRendererReference) -> Vec<SpriteInstance> {
        let mut sprites = vec![];

        for element in self.elements.iter() {
            let sprite = sprite_of(&element.gate).unwrap();
            let sprite_handle = *sheets.get_sprite(GATE_SHEET, sprite).unwrap();

            let sprite_instance = SpriteInstance {
                sprite_handle,
                position: element.position,
                scale: 1.0,
                color: glam::Vec4::splat(1.0),
            };

            sprites.push(sprite_instance);
        }
        sprites
    }

    pub fn connection_instances(&self) -> Vec<CubicBezier> {
        let mut beziers = vec![];

        for connection in self.connections.iter() {
            let from_elm = &self[connection.from.0];
            let from = from_elm.gate.output_offset() + from_elm.position;

            let to_elm = &self[connection.to.0];
            let to = to_elm.gate.input_offsets()[connection.to.1 .0] + to_elm.position;

            beziers.push(CubicBezier::between_points(from, to));
        }

        beziers
    }
}
