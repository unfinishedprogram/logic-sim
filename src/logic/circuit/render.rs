use glam::{Vec2, Vec4};
use lyon::tessellation::VertexBuffers;

#[cfg(feature = "rayon")]
use crate::render::helpers::join_buffers;
#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::{
    super::gate::Gate,
    connection::{ElementIdx, IOSpecifier},
    Circuit, CircuitElement,
};
use crate::{
    assets,
    game::GameInput,
    logic::hit_test::HitTestResult,
    render::{
        frame::Frame, helpers::extend_vertex_buffer, line::cubic_bezier::CubicBezier,
        vertex::VertexUV,
    },
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
        let tolerance = f32::max(0.001 * frame.camera().size.x, 0.001);

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

        let filter_map = |conn| {
            let line = self.cubic_bezier_from_connection(conn);
            if frame.camera().bounds().overlaps(&line.bounds()) {
                Some((
                    line,
                    self.solver.output_results[conn.from.0 .0] as u8 as f32,
                ))
            } else {
                None
            }
        };

        let fold = |mut vb, (line, color): (CubicBezier, f32)| {
            line.tesselate(&mut vb, 0.05, Vec4::new(0.0, color, 0.0, 1.0), tolerance);
            vb
        };

        #[cfg(not(feature = "rayon"))]
        let buffers = {
            self.connections
                .iter()
                .filter_map(filter_map)
                .fold(VertexBuffers::<VertexUV, u32>::new(), fold)
        };

        #[cfg(feature = "rayon")]
        let buffers = {
            join_buffers(
                self.connections
                    .par_iter()
                    .filter_map(filter_map)
                    .fold_with(VertexBuffers::<VertexUV, u32>::new(), fold)
                    .collect(),
            )
        };

        extend_vertex_buffer(frame.line_geo_buffers(), buffers);

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
                frame.line_geo_buffers(),
                0.05,
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                tolerance,
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
