use super::circuit::connection::{ElementIdx, InputSpecifier, OutputSpecifier};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitTestResult {
    Element(ElementIdx),
    Input(InputSpecifier),
    Output(OutputSpecifier),
}
