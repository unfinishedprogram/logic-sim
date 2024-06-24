use super::circuit::connection::{ConnectionIdx, ElementIdx, IOSpecifier};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitTestResult {
    Element(ElementIdx),
    IO(IOSpecifier),
    Connection(ConnectionIdx),
}
