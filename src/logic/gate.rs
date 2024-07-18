use util::bounds::Bounds;

use super::circuit::connection::Connectable;
use glam::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gate {
    Button(bool),
    Input(bool),
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
}

impl Gate {
    const INPUT_OFFSETS_2: [Vec2; 2] = [Vec2::new(-0.3, -0.2), Vec2::new(-0.3, 0.2)];
    const INPUT_OFFSETS_1: [Vec2; 1] = [Vec2::new(-0.3, 0.0)];
    const INPUT_OFFSETS_0: [Vec2; 0] = [];

    pub const fn input_offsets(&self) -> &'static [Vec2] {
        match self.input_count() {
            0 => &Self::INPUT_OFFSETS_0,
            1 => &Self::INPUT_OFFSETS_1,
            2 => &Self::INPUT_OFFSETS_2,
            _ => unreachable!(),
        }
    }

    pub const fn input_count(&self) -> usize {
        match self {
            Self::Input(_) | Self::Button(_) | Self::Off | Self::On => 0,
            Self::Not | Self::Buf => 1,
            Self::And | Self::Or | Self::Xor | Self::Nand | Self::Nor | Self::Xnor => 2,
        }
    }

    const OUTPUT_OFFSET: Vec2 = Vec2::new(0.4, 0.0);
    pub const fn output_offset(&self) -> Vec2 {
        Self::OUTPUT_OFFSET
    }

    pub fn bounds(&self) -> Bounds {
        let size = 0.25;
        let offset = Vec2::splat(size);
        Bounds::new(-offset, offset)
    }
}

impl Connectable for Gate {
    fn inputs(&self) -> usize {
        match self {
            Self::Input(_) => 0,
            Self::Not | Self::Buf => 1,
            _ => 2,
        }
    }

    fn outputs(&self) -> usize {
        1
    }
}
