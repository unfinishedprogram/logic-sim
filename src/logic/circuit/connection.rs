#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputSpecifier {
    pub element_idx: usize,
    pub output_idx: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputSpecifier {
    pub element_idx: usize,
    pub input_idx: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Connection {
    pub from: OutputSpecifier,
    pub to: InputSpecifier,
}

pub trait Connectable {
    fn inputs(&self) -> usize;
    fn outputs(&self) -> usize;
}
