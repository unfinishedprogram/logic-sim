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
    render::{frame::Frame, line::cubic_bezier::CubicBezier, vector::VectorInstance},
};

pub fn sprite_of(gate: &Gate) -> Option<&'static str> {
    match gate {
        Gate::Input(_) => None,
        Gate::And => Some(assets::svg::gates::AND),
        Gate::Or => Some(assets::svg::gates::OR),
        Gate::Not => Some(assets::svg::gates::NOT),
        Gate::Buf => Some(assets::svg::gates::BUF),
        Gate::Xor => Some(assets::svg::gates::XOR),
        Gate::Nand => Some(assets::svg::gates::NAND),
        Gate::Nor => Some(assets::svg::gates::NOR),
        Gate::Xnor => Some(assets::svg::gates::XNOR),
    }
}

impl CircuitElement {
    pub fn draw(&self, active: bool, frame: &mut Frame) {
        let sprite = sprite_of(&self.gate).unwrap();
        frame.draw_vector_lazy(sprite, self.position, Vec4::ONE, Vec2::ONE)

        // if active {
        //     let outlined = format!("{}_outline", sprite);
        //     let vector_handle = *frame.assets.vectors.get_vector(&outlined).unwrap();
        //     frame.draw_vector(
        //         VectorInstance::new(vector_handle)
        //             .with_transform(self.position)
        //             .with_color(Vec4::new(0.2, 0.2, 1.0, 1.0)),
        //     );
        // }
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
            let from_elm = &self[connection.from.0];
            let from = from_elm.gate.output_offset() + from_elm.position;

            let to_elm = &self[connection.to.0];
            let to = to_elm.gate.input_offsets()[connection.to.1 .0] + to_elm.position;

            let line = CubicBezier::between_points(from, to);

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
            let dot_source = assets::svg::DOT;

            for dot in self.connection_dots() {
                let position = self.io_position(dot);
                let color = match dot {
                    IOSpecifier::Input(_) => Vec4::new(1.0, 0.0, 0.0, 1.0),
                    IOSpecifier::Output(_) => Vec4::new(0.0, 1.0, 0.0, 1.0),
                };

                let scale = match game_input.hot {
                    Some(HitTestResult::IO(hot_dot)) if hot_dot == dot => Vec2::splat(1.2),
                    _ => Vec2::splat(1.0),
                };

                frame.draw_vector_lazy(dot_source, position, color, scale);
            }
        }
    }
}
