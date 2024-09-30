pub mod connection;
mod edit_circuit;
pub mod embedded;
mod examples;
pub use edit_circuit::EditCircuit;
use embedded::EmbeddedCircuit;
mod element;
mod render;

#[cfg(test)]
mod test;

use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

use element::CircuitElement;
use glam::{vec2, Vec2};

use super::{gate::Gate, hit_test::HitTestResult, solver::SolverState};
use crate::render::line::cubic_bezier::CubicBezier;

use util::bounds::Bounds;

use connection::{
    Connection, ConnectionIdx, ElementIdx, IOSpecifier, InputIdx, InputSpecifier, OutputIdx,
    OutputSpecifier,
};

#[derive(Default, Clone, Debug)]
pub struct Circuit {
    // TODO: Make this generic
    pub(crate) elements: Vec<CircuitElement>,
    pub(crate) connections: Vec<Connection>,
    pub(crate) solver: SolverState,
}

impl Circuit {
    // Progress a single clock cycle
    pub fn step(&mut self) {
        let solver = self.solver.clone();
        let solver = solver.step(self);
        self.solver = solver;
    }

    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    pub fn extreme_test_circuit() -> Self {
        let mut circuit = Circuit::default();

        for _ in 0..10000 {
            circuit.add_random_component()
        }

        for _ in 0..10000 {
            circuit.add_random_connection()
        }

        circuit
    }

    pub fn embed(&self) -> EmbeddedCircuit {
        EmbeddedCircuit::new(self.clone()).unwrap()
    }

    fn add_random_component(&mut self) {
        let position = vec2(rand::random::<f32>() * 100.0, rand::random::<f32>() * 100.0);
        let gates = [
            Gate::And,
            Gate::Or,
            Gate::Not,
            Gate::Buf,
            Gate::Xor,
            Gate::Xnor,
            Gate::Nand,
        ];
        let gate = gates[rand::random::<usize>() % gates.len()].clone();

        self.add_gate(gate, position);
    }

    fn add_random_connection(&mut self) {
        let from = OutputSpecifier(
            ElementIdx(rand::random::<usize>() % self.elements.len()),
            OutputIdx(0),
        );

        let to = InputSpecifier(
            ElementIdx(rand::random::<usize>() % self.elements.len()),
            InputIdx(0),
        );

        self.add_connection(from.to(to));
    }

    pub fn add_gate(&mut self, gate: Gate, position: Vec2) -> ElementIdx {
        let idx = ElementIdx(self.elements.len());
        self.elements.push(CircuitElement { gate, position });
        idx
    }

    pub fn add_connection(&mut self, connection: Connection) {
        if self.connections.contains(&connection) {
            return;
        }
        self.connections.push(connection);
    }

    pub fn remove_gate(&mut self, ElementIdx(index): ElementIdx) {
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

    // Some gates will change state based on click events
    pub fn click_gate(&mut self, ElementIdx(index): ElementIdx) {
        println!("Clicked gate {}", index);

        match &mut self.elements[index].gate {
            Gate::Button(state) => *state = true,
            Gate::Const(state) => *state = !*state,
            _ => {}
        }
    }

    pub fn remove_connections(&mut self, spec: impl Into<IOSpecifier>) {
        match spec.into() {
            IOSpecifier::Input(input) => {
                self.connections.retain(|connection| connection.to != input);
            }
            IOSpecifier::Output(output) => {
                self.connections
                    .retain(|connection| connection.from != output);
            }
        }
    }

    pub fn remove_many_connections(&mut self, connections: HashSet<ConnectionIdx>) {
        let mut index: usize = 0;
        self.connections.retain(|_| {
            index += 1;
            !connections.contains(&ConnectionIdx(index - 1))
        });
    }

    pub fn remove_connection(&mut self, idx: ConnectionIdx) {
        self.connections.remove(idx.0);
    }

    pub fn hit_test_bounds(&self, bounds: Bounds) -> HashSet<HitTestResult> {
        let mut res = vec![];

        for (element_idx, element) in self.elements.iter().enumerate() {
            for (input_idx, offset) in element.gate.input_offsets().into_iter().enumerate() {
                let element_bounds =
                    Bounds::from_center_and_size(element.position + offset, vec2(0.1, 0.1));

                if element_bounds.overlaps(&bounds) {
                    res.push(HitTestResult::IO(
                        InputSpecifier(ElementIdx(element_idx), InputIdx(input_idx)).into(),
                    ))
                }
            }

            for (output_idx, offset) in element.gate.output_offsets().into_iter().enumerate() {
                let element_bounds =
                    Bounds::from_center_and_size(element.position + offset, vec2(0.1, 0.1));

                if element_bounds.overlaps(&bounds) {
                    res.push(HitTestResult::IO(
                        OutputSpecifier(ElementIdx(element_idx), OutputIdx(output_idx)).into(),
                    ));
                }
            }
        }

        for (element_idx, element) in self.elements.iter().enumerate() {
            if element.bounds().overlaps(&bounds) {
                res.push(HitTestResult::Element(ElementIdx(element_idx)))
            }
        }

        for (connection_idx, connection) in self.connections.iter().enumerate() {
            if self
                .cubic_bezier_from_connection(connection)
                .hit_test_bounds(bounds, 0.05)
            {
                res.push(HitTestResult::Connection(ConnectionIdx(connection_idx)))
            }
        }

        HashSet::from_iter(res)
    }

    pub fn hit_test(&self, position: Vec2) -> Option<HitTestResult> {
        for (element_idx, element) in self.elements.iter().enumerate() {
            for (input_idx, offset) in element.gate.input_offsets().into_iter().enumerate() {
                let bounds =
                    Bounds::from_center_and_size(element.position + offset, vec2(0.1, 0.1));

                if bounds.contains(position) {
                    return Some(HitTestResult::IO(
                        InputSpecifier(ElementIdx(element_idx), InputIdx(input_idx)).into(),
                    ));
                }
            }

            for (output_idx, offset) in element.gate.output_offsets().into_iter().enumerate() {
                let bounds =
                    Bounds::from_center_and_size(element.position + offset, vec2(0.1, 0.1));

                if bounds.contains(position) {
                    return Some(HitTestResult::IO(
                        OutputSpecifier(ElementIdx(element_idx), OutputIdx(output_idx)).into(),
                    ));
                }
            }
        }

        for (element_idx, element) in self.elements.iter().enumerate() {
            if element.hit_test(position) {
                return Some(HitTestResult::Element(ElementIdx(element_idx)));
            }
        }

        for (connection_idx, connection) in self.connections.iter().enumerate() {
            if self
                .cubic_bezier_from_connection(connection)
                .hit_test(position, 0.05)
            {
                return Some(HitTestResult::Connection(ConnectionIdx(connection_idx)));
            }
        }

        None
    }

    fn connection_dots(&self) -> impl Iterator<Item = IOSpecifier> + '_ {
        self.elements
            .iter()
            .enumerate()
            .map(|(element_idx, element)| (ElementIdx(element_idx), element))
            .flat_map(|(element_idx, element)| {
                element
                    .gate
                    .input_offsets()
                    .into_iter()
                    .enumerate()
                    .map(move |(input_idx, _)| {
                        IOSpecifier::Input(InputSpecifier(element_idx, InputIdx(input_idx)))
                    })
                    .chain(element.gate.output_offsets().into_iter().enumerate().map(
                        move |(output_idx, _)| {
                            IOSpecifier::Output(OutputSpecifier(element_idx, OutputIdx(output_idx)))
                        },
                    ))
            })
    }

    fn io_position(&self, spec: impl Into<IOSpecifier>) -> Vec2 {
        let spec = spec.into();
        let element = &self[spec.element()];
        let offset = match spec {
            IOSpecifier::Input(InputSpecifier(_, input_idx)) => {
                element.gate.input_offset(input_idx)
            }
            IOSpecifier::Output(OutputSpecifier(_, output_idx)) => {
                element.gate.output_offset(output_idx)
            }
        };

        element.position + offset
    }

    pub fn cubic_bezier_from_connection(&self, connection: &Connection) -> CubicBezier {
        let from_elm = &self[connection.from.0];
        let from = from_elm.gate.output_offset(connection.from.1) + from_elm.position;
        let to_elm = &self[connection.to.0];
        let to = to_elm.gate.input_offset(connection.to.1) + to_elm.position;

        CubicBezier::between_points(from, to)
    }

    pub fn output_value(&self, io: OutputSpecifier) -> bool {
        // TODO: Make this handle multiple outputs
        self.solver.output_results.read_output(io)
    }

    pub fn right_size_solver(&mut self) {
        self.solver.set_size(self.elements.len());
    }
}

impl Index<ElementIdx> for Circuit {
    type Output = CircuitElement;

    fn index(&self, index: ElementIdx) -> &Self::Output {
        &self.elements[index.0]
    }
}

impl IndexMut<ElementIdx> for Circuit {
    fn index_mut(&mut self, index: ElementIdx) -> &mut Self::Output {
        &mut self.elements[index.0]
    }
}

impl Index<ConnectionIdx> for Circuit {
    type Output = Connection;

    fn index(&self, index: ConnectionIdx) -> &Self::Output {
        &self.connections[index.0]
    }
}

impl IndexMut<ConnectionIdx> for Circuit {
    fn index_mut(&mut self, index: ConnectionIdx) -> &mut Self::Output {
        &mut self.connections[index.0]
    }
}
