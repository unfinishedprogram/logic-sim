use glam::{Vec2, Vec4};

use super::{
    super::gate::Gate,
    connection::{ElementIdx, IOSpecifier},
    Circuit, CircuitElement,
};
use crate::{
    assets,
    game::GameInput,
    logic::hit_test::HitTestResult,
    render::{frame::Frame, line::cubic_bezier::CubicBezier},
};

pub fn sprite_of(gate: &Gate, active: bool) -> Option<&'static str> {
    use assets::svg::gates;
    match (gate, active) {
        (Gate::Input(_), _) => None,

        (Gate::And, true) => Some(&gates::AND_ACTIVE),
        (Gate::And, false) => Some(&gates::AND_NORMAL),

        (Gate::Or, true) => Some(&gates::OR_ACTIVE),
        (Gate::Or, false) => Some(&gates::OR_NORMAL),

        (Gate::Not, true) => Some(&gates::NOT_ACTIVE),
        (Gate::Not, false) => Some(&gates::NOT_NORMAL),

        (Gate::Xor, true) => Some(&gates::XOR_ACTIVE),
        (Gate::Xor, false) => Some(&gates::XOR_NORMAL),

        (Gate::Nand, true) => Some(&gates::NAND_ACTIVE),
        (Gate::Nand, false) => Some(&gates::NAND_NORMAL),

        (Gate::Nor, true) => Some(&gates::NOR_ACTIVE),
        (Gate::Nor, false) => Some(&gates::NOR_NORMAL),

        (Gate::Xnor, true) => Some(&gates::XNOR_ACTIVE),
        (Gate::Xnor, false) => Some(&gates::XNOR_NORMAL),

        (Gate::Buf, true) => Some(&gates::BUF_ACTIVE),
        (Gate::Buf, false) => Some(&gates::BUF_NORMAL),
    }
}

impl CircuitElement {
    pub fn draw(&self, active: bool, frame: &mut Frame) {
        let sprite = sprite_of(&self.gate, active).unwrap();
        frame.draw_vector_lazy(sprite, self.position, Vec4::ONE, Vec2::ONE, active as u16)
    }
}

impl Circuit {
    pub fn draw(&self, frame: &mut Frame, game_input: &GameInput) {
        for (idx, element) in self.elements.iter().enumerate() {
            element.draw(
                if let Some(HitTestResult::Element(ElementIdx(hot_idx))) = game_input.hot {
                    hot_idx == idx
                } else {
                    false
                },
                frame,
            );
        }

        for connection in self.connections.iter() {
            let line = self.cubic_bezier_from_connection(connection);
            let color = self.solver.output_results[connection.from.0 .0] as u8 as f32 * 1.0;

            line.tesselate(
                0.05,
                Vec4::new(0.0, color, 0.0, 1.0),
                frame.line_geo_buffers(),
            );
        }

        // Draw connection preview while being made
        if let Some(source_point) = match game_input.active {
            Some(HitTestResult::IO(IOSpecifier::Input(input))) => {
                let from_elm = &self[input.0];
                Some(from_elm.gate.input_offsets()[input.1 .0] + from_elm.position)
            }
            Some(HitTestResult::IO(IOSpecifier::Output(output))) => {
                let from_elm = &self[output.0];
                Some(from_elm.gate.output_offset() + from_elm.position)
            }
            _ => None,
        } {
            let to = frame.input().mouse_world_position;
            let line = CubicBezier::between_points(source_point, to);
            line.tesselate(
                0.05,
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                frame.line_geo_buffers(),
            );
        }

        {
            for dot in self.connection_dots() {
                let position = self.io_position(dot);
                let dot_source = match dot {
                    IOSpecifier::Input(_) => &assets::svg::DOT_INPUT,
                    IOSpecifier::Output(_) => &assets::svg::DOT_OUTPUT,
                };

                let scale = match game_input.hot {
                    Some(HitTestResult::IO(hot_dot)) if hot_dot == dot => Vec2::splat(1.2),
                    _ => Vec2::splat(1.0),
                };

                frame.draw_vector_lazy(dot_source, position, Vec4::ONE, scale, 2);
            }
        }
    }
}
