use std::{iter::once, ops::Index};

use glam::{vec2, Vec2};

use super::{gate::Gate, hit_test::HitTestResult};
use crate::util::bounds::Bounds;

pub mod connection;
mod render;
use connection::{Connection, ElementIdx, InputIdx, InputSpecifier, OutputIdx, OutputSpecifier};

#[derive(Default)]
pub struct Circuit {
    // TODO: Make this generic
    elements: Vec<CircuitElement>,
    connections: Vec<Connection>,
}

pub struct CircuitElement {
    gate: Gate,
    position: Vec2,
}

pub enum ConnectionDotRefType {
    Input,
    Output,
}
pub struct ConnectionDotRef {
    pub position: Vec2,
    pub ty: ConnectionDotRefType,
    pub element_idx: usize,
    pub connection_idx: usize,
}

impl Circuit {
    pub fn test_circuit() -> Self {
        let mut circuit = Circuit::default();

        let a = circuit.add_gate(Gate::Buf, vec2(0.0, 0.0));
        let b = circuit.add_gate(Gate::Buf, vec2(0.0, 1.0));

        let xor = circuit.add_gate(Gate::Xor, vec2(3.0, 0.0));
        let and = circuit.add_gate(Gate::And, vec2(3.0, 1.0));

        circuit.add_connection(a.to(InputSpecifier(xor, InputIdx(0))));
        circuit.add_connection(a.to(InputSpecifier(and, InputIdx(0))));

        circuit.add_connection(b.to(InputSpecifier(xor, InputIdx(1))));
        circuit.add_connection(b.to(InputSpecifier(and, InputIdx(1))));

        circuit
    }

    pub fn add_gate(&mut self, gate: Gate, position: Vec2) -> ElementIdx {
        let idx = ElementIdx(self.elements.len());
        self.elements.push(CircuitElement { gate, position });
        idx
    }

    fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    pub fn remove_gate(&mut self, index: usize) {
        // Remove connections referencing the removed gate
        self.connections
            .retain(|connection| connection.from.0 .0 != index && connection.to.0 .0 != index);

        // Modify the indices of the remaining connections which come after the removed gate
        for connection in self.connections.iter_mut() {
            if connection.from.0 .0 > index {
                connection.from.0 .0 -= 1;
            }

            if connection.to.0 .0 > index {
                connection.to.0 .0 -= 1;
            }
        }

        // Finally remove the element
        self.elements.remove(index);
    }

    pub fn hit_test(&self, position: Vec2) -> Option<HitTestResult> {
        for (element_idx, element) in self.elements.iter().enumerate() {
            for (input_idx, offset) in element.gate.input_offsets().iter().enumerate() {
                let bounds =
                    Bounds::from_center_and_size(element.position + *offset, vec2(0.1, 0.1));

                if bounds.contains(position) {
                    return Some(HitTestResult::Input(InputSpecifier(
                        ElementIdx(element_idx),
                        InputIdx(input_idx),
                    )));
                }
            }

            for (output_idx, offset) in once(element.gate.output_offset()).enumerate() {
                let bounds =
                    Bounds::from_center_and_size(element.position + offset, vec2(0.1, 0.1));

                if bounds.contains(position) {
                    return Some(HitTestResult::Output(OutputSpecifier(
                        ElementIdx(element_idx),
                        OutputIdx(output_idx),
                    )));
                }
            }
        }

        for (element_idx, element) in self.elements.iter().enumerate() {
            let bounds: Bounds = element.gate.bounds().translate(element.position);

            if bounds.contains(position) {
                return Some(HitTestResult::Element(ElementIdx(element_idx)));
            }
        }

        None
    }

    pub fn connection_dots(&self) -> impl Iterator<Item = ConnectionDotRef> + '_ {
        self.elements
            .iter()
            .enumerate()
            .flat_map(|(element_idx, element)| {
                element
                    .gate
                    .input_offsets()
                    .iter()
                    .map(move |offset| (*offset, ConnectionDotRefType::Input))
                    .chain(once((
                        element.gate.output_offset(),
                        ConnectionDotRefType::Output,
                    )))
                    .enumerate()
                    .map(move |(connection_idx, (offset, ty))| ConnectionDotRef {
                        position: element.position + offset,
                        ty,
                        element_idx,
                        connection_idx,
                    })
            })
    }

    pub fn input_position(&self, spec: InputSpecifier) -> Vec2 {
        let input_offset = self[spec.0].gate.input_offsets()[spec.1 .0];
        let element_position = self[spec.0].position;

        input_offset + element_position
    }

    pub fn output_position(&self, spec: OutputSpecifier) -> Vec2 {
        let output_offset = self[spec.0].gate.output_offset();
        let element_position = self[spec.0].position;

        output_offset + element_position
    }
}

impl Index<ElementIdx> for Circuit {
    type Output = CircuitElement;

    fn index(&self, index: ElementIdx) -> &Self::Output {
        &self.elements[index.0]
    }
}
