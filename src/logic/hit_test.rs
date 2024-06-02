use super::circuit::connection::{ElementIdx, IOSpecifier, InputSpecifier, OutputSpecifier};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitTestResult {
    Element(ElementIdx),
    IO(IOSpecifier),
}
