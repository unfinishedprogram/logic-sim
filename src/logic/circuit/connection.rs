#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputSpecifier(pub ElementIdx, pub OutputIdx);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputSpecifier(pub ElementIdx, pub InputIdx);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSpecifier {
    Input(InputSpecifier),
    Output(OutputSpecifier),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Connection {
    pub from: OutputSpecifier,
    pub to: InputSpecifier,
}

impl ElementIdx {
    pub fn to(self, other: InputSpecifier) -> Connection {
        Connection {
            from: OutputSpecifier(self, OutputIdx(0)),
            to: other,
        }
    }
}

impl OutputSpecifier {
    pub fn to(self, other: InputSpecifier) -> Connection {
        Connection {
            from: self,
            to: other,
        }
    }
}

pub trait Connectable {
    fn inputs(&self) -> usize;
    fn outputs(&self) -> usize;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElementIdx(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputIdx(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputIdx(pub usize);
