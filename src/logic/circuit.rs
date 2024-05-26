use glam::Vec2;

use super::{gate::Gate, hit_test::HitTestResult};
use crate::util::bounds::Bounds;

pub mod connection;
mod render;
use connection::Connection;

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

impl Circuit {
    pub fn test_circuit() -> Self {
        let mut circuit = Circuit::default();

        circuit.add_gate(Gate::And, Vec2::new(0.0, 0.0));
        circuit.add_gate(Gate::Or, Vec2::new(2.0, 2.0));
        circuit.add_gate(Gate::Not, Vec2::new(3.0, 3.0));

        circuit.add_connection(Connection {
            from: connection::OutputSpecifier {
                element_idx: 0,
                output_idx: 0,
            },
            to: connection::InputSpecifier {
                element_idx: 1,
                input_idx: 0,
            },
        });

        circuit.add_connection(Connection {
            from: connection::OutputSpecifier {
                element_idx: 1,
                output_idx: 0,
            },
            to: connection::InputSpecifier {
                element_idx: 2,
                input_idx: 0,
            },
        });

        circuit
    }

    pub fn add_gate(&mut self, gate: Gate, position: Vec2) {
        self.elements.push(CircuitElement { gate, position });
    }

    fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    pub fn remove_gate(&mut self, index: usize) {
        // Remove connections referencing the removed gate
        self.connections.retain(|connection| {
            connection.from.element_idx != index && connection.to.element_idx != index
        });

        // Modify the indices of the remaining connections which come after the removed gate
        for connection in self.connections.iter_mut() {
            if connection.from.element_idx > index {
                connection.from.element_idx -= 1;
            }

            if connection.to.element_idx > index {
                connection.to.element_idx -= 1;
            }
        }

        // Finally remove the element
        self.elements.remove(index);
    }

    pub fn hit_test(&self, position: Vec2) -> HitTestResult {
        for (i, element) in self.elements.iter().enumerate() {
            let bounds: Bounds = element.gate.bounds().translate(element.position);

            if bounds.contains(position) {
                return HitTestResult::Element(i);
            }
        }

        HitTestResult::None
    }
}
