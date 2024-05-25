use super::circuit::connection::Connectable;

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
