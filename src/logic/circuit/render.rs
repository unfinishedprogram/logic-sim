use crate::render::{frame::Frame, line::cubic_bezier::CubicBezier};

use super::{super::gate::Gate, Circuit, CircuitElement};

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

impl CircuitElement {
    pub fn draw(&self, frame: &mut Frame) {
        let sprite = sprite_of(&self.gate).unwrap();
        let sprite_handle = *frame.assets.sprites.get_sprite(GATE_SHEET, sprite).unwrap();
        frame.draw_sprite(sprite_handle, self.position, 1.0);
    }
}

impl Circuit {
    pub fn draw(&self, frame: &mut Frame) {
        for element in &self.elements {
            element.draw(frame);
        }

        for connection in self.connections.iter() {
            let from_elm = &self[connection.from.0];
            let from = from_elm.gate.output_offset() + from_elm.position;

            let to_elm = &self[connection.to.0];
            let to = to_elm.gate.input_offsets()[connection.to.1 .0] + to_elm.position;

            let line = CubicBezier::between_points(from, to);

            for line in line.as_line_geometries(10, 0.05) {
                frame.draw_line(line);
            }
        }
    }
}
