use util::bounds::Bounds;

use super::circuit::{
    connection::{InputIdx, OutputIdx},
    embedded::EmbeddedCircuit,
};
use glam::Vec2;

#[derive(Clone, Debug)]
pub enum Gate {
    Button(bool),
    Const(bool),
    And,
    Or,
    Not,
    Buf,
    Xor,
    Nand,
    Nor,
    Xnor,
    On,
    Off,
    Input(Option<String>),
    Output(Option<String>),
    Embedded(EmbeddedCircuit),
}

impl Gate {
    fn io_offsets(
        x_offset: f32,
        vertical_step: f32,
        count: usize,
    ) -> impl IntoIterator<Item = Vec2> {
        let vertical_offset = if count & 1 == 1 {
            0.0
        } else {
            -vertical_step / 2.0
        };

        let top = vertical_step * (count / 2) as f32 + vertical_offset;

        (0..count).map(move |i| Vec2::new(x_offset, top - vertical_step * i as f32))
    }

    pub fn input_offsets(&self) -> impl IntoIterator<Item = Vec2> {
        let x_offset = -0.3;
        let vertical_step = 0.4;
        Self::io_offsets(x_offset, vertical_step, self.input_count())
    }

    pub fn output_offsets(&self) -> impl IntoIterator<Item = Vec2> + '_ {
        let x_offset = 0.4;
        let vertical_step = 0.4;
        Self::io_offsets(x_offset, vertical_step, self.output_count())
    }

    pub fn output_offset(&self, OutputIdx(index): OutputIdx) -> Vec2 {
        self.output_offsets().into_iter().nth(index).unwrap()
    }

    pub fn input_offset(&self, InputIdx(index): InputIdx) -> Vec2 {
        self.input_offsets().into_iter().nth(index).unwrap()
    }

    pub fn input_count(&self) -> usize {
        match self {
            Self::Const(_) | Self::Button(_) | Self::Off | Self::On | Self::Input(_) => 0,
            Self::Not | Self::Buf | Self::Output(_) => 1,
            Self::And | Self::Or | Self::Xor | Self::Nand | Self::Nor | Self::Xnor => 2,
            Self::Embedded(embed) => embed.input_count(),
        }
    }

    pub fn output_count(&self) -> usize {
        match self {
            Gate::Embedded(embed) => embed.output_count(),
            _ => 1,
        }
    }

    pub fn bounds(&self) -> Bounds {
        let size = 0.25;
        let offset = Vec2::splat(size);
        Bounds::new(-offset, offset)
    }
}

impl From<EmbeddedCircuit> for Gate {
    fn from(embed: EmbeddedCircuit) -> Self {
        Self::Embedded(embed)
    }
}
