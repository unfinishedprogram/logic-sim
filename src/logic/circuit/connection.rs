#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OutputSpecifier(pub ElementIdx, pub OutputIdx);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputSpecifier(pub ElementIdx, pub InputIdx);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IOSpecifier {
    Input(InputSpecifier),
    Output(OutputSpecifier),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Connection {
    pub from: OutputSpecifier,
    pub to: InputSpecifier,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ElementIdx(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputIdx(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OutputIdx(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConnectionIdx(pub usize);

impl IOSpecifier {
    pub fn element(&self) -> ElementIdx {
        match self {
            IOSpecifier::Input(InputSpecifier(element, _)) => *element,
            IOSpecifier::Output(OutputSpecifier(element, _)) => *element,
        }
    }
}

impl From<OutputSpecifier> for IOSpecifier {
    fn from(output: OutputSpecifier) -> Self {
        IOSpecifier::Output(output)
    }
}

impl From<InputSpecifier> for IOSpecifier {
    fn from(input: InputSpecifier) -> Self {
        IOSpecifier::Input(input)
    }
}

impl ElementIdx {
    pub fn input(self, idx: usize) -> InputSpecifier {
        InputSpecifier(self, InputIdx(idx))
    }

    pub fn output(self, idx: usize) -> OutputSpecifier {
        OutputSpecifier(self, OutputIdx(idx))
    }
}
