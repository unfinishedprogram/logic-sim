use crate::util::bounds::Bounds;

use super::circuit::connection::Connectable;
use glam::Vec2;

pub enum Gate {
    Input(bool),
    And,
    Or,
    Not,
    Buf,
    Xor,
    Nand,
    Nor,
    Xnor,
}

impl Gate {
    const INPUT_OFFSETS_AND: [Vec2; 2] = [Vec2::new(-0.3, -0.2), Vec2::new(-0.3, 0.2)];
    const INPUT_OFFSETS_NOT: [Vec2; 1] = [Vec2::new(-0.3, 0.0)];
    const INPUT_OFFSETS_EMPTY: [Vec2; 0] = [];

    pub const fn input_offsets(&self) -> &'static [Vec2] {
        match self {
            Self::Input(_) => &Self::INPUT_OFFSETS_EMPTY,
            Self::And | Self::Or | Self::Xor | Self::Nand | Self::Nor | Self::Xnor => {
                &Self::INPUT_OFFSETS_AND
            }
            Self::Not | Self::Buf => &Self::INPUT_OFFSETS_NOT,
        }
    }

    const OUTPUT_OFFSET: Vec2 = Vec2::new(0.4, 0.0);
    pub const fn output_offset(&self) -> Vec2 {
        Self::OUTPUT_OFFSET
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(Vec2::new(-0.5, -0.5), Vec2::new(0.5, 0.5))
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
