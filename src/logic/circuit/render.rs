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

            let sprite_instance = sheets
                .get_sprite(GATE_SHEET, sprite)
                .unwrap()
                .instantiate(element.position, 1.0);

            sprites.push(sprite_instance);
        }
        sprites
    }

    pub fn connection_instances(&self) -> Vec<CubicBezier> {
        let mut beziers = vec![];

        for connection in self.connections.iter() {
            let from = self.elements[connection.from.element_idx]
                .gate
                .output_offset()
                + self.elements[connection.from.element_idx].position;
            let to = self.elements[connection.to.element_idx]
                .gate
                .input_offsets()[connection.to.input_idx]
                + self.elements[connection.to.element_idx].position;

            beziers.push(CubicBezier::between_points(from, to));
        }

        beziers
    }
}
