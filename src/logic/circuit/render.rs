use glam::{Vec2, Vec4};

use super::{super::gate::Gate, connection::IOSpecifier, Circuit, CircuitElement};
use crate::{
    logic::hit_test::HitTestResult,
    render::{frame::Frame, line::cubic_bezier::CubicBezier, vector},
};

pub fn sprite_of(gate: &Gate) -> Option<&'static str> {
    match gate {
        Gate::Input(_) => None,
        Gate::And => Some("and"),
        Gate::Or => Some("or"),
        Gate::Not => Some("not"),
        Gate::Buf => Some("buf"),
        Gate::Xor => Some("xor"),
        Gate::Nand => Some("nand"),
        Gate::Nor => Some("nor"),
        Gate::Xnor => Some("xnor"),
    }
}

impl CircuitElement {
    pub fn draw(&self, frame: &mut Frame) {
        let sprite = sprite_of(&self.gate).unwrap();
        let vector_handle = *frame.assets.vectors.get_vector(sprite).unwrap();
        frame.draw_vector(vector::Instance::new(vector_handle).with_transform(self.position));
    }
}

impl Circuit {
    pub fn draw(&self, frame: &mut Frame, hot: &Option<HitTestResult>) {
        for element in &self.elements {
            element.draw(frame);
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

        {
            let dot_object =
                vector::Instance::new(*frame.assets.vectors.get_vector("dot").unwrap());

            for dot in self.connection_dots() {
                let position = self.io_position(dot);
                let color = match dot {
                    IOSpecifier::Input(_) => Vec4::new(1.0, 0.0, 0.0, 1.0),
                    IOSpecifier::Output(_) => Vec4::new(0.0, 1.0, 0.0, 1.0),
                };

                let scale = match hot {
                    Some(HitTestResult::IO(hot_dot)) if *hot_dot == dot => Vec2::splat(1.2),
                    _ => Vec2::splat(1.0),
                };

                frame.draw_vector(
                    dot_object
                        .with_color(color)
                        .with_transform(position)
                        .with_scale(scale),
                );
            }
        }
    }
}
