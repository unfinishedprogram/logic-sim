pub mod connection;
mod element;
mod render;
pub mod selection;

use std::{
    iter::once,
    ops::{Index, IndexMut},
};

use element::CircuitElement;
use glam::{vec2, Vec2};
use selection::ElementSelection;

use super::{gate::Gate, hit_test::HitTestResult, solver::SolverState};
use crate::{
    game::{input::InputState, GameInput},
    render::line::cubic_bezier::CubicBezier,
};

use util::bounds::Bounds;

use connection::{
    Connection, ConnectionIdx, ElementIdx, IOSpecifier, InputIdx, InputSpecifier, OutputIdx,
    OutputSpecifier,
};

#[derive(Default)]
pub struct Circuit {
    // TODO: Make this generic
    pub(crate) elements: Vec<CircuitElement>,
    pub(crate) connections: Vec<Connection>,
    pub(crate) solver: SolverState,
    pub(crate) selection: ElementSelection,
}

impl Circuit {
    // Progress a single clock cycle
    pub fn step(&mut self) {
        let solver = self.solver.clone();
        let solver = solver.step(self);
        self.solver = solver;

        for element in self.elements.iter_mut() {
            if let Gate::Button(state) = &mut element.gate {
                *state = false;
            }
        }
    }

    pub fn test_circuit() -> Self {
        let mut circuit = Circuit::default();

        let a = circuit.add_gate(Gate::Buf, vec2(0.0, 0.0));
        let b = circuit.add_gate(Gate::Not, vec2(0.0, 1.0));

        let xor = circuit.add_gate(Gate::Xor, vec2(3.0, 0.0));
        let and = circuit.add_gate(Gate::And, vec2(3.0, 1.0));

        circuit.add_connection(a.to(InputSpecifier(xor, InputIdx(0))));
        circuit.add_connection(a.to(InputSpecifier(and, InputIdx(0))));

        circuit.add_connection(b.to(InputSpecifier(xor, InputIdx(1))));
        circuit.add_connection(b.to(InputSpecifier(and, InputIdx(1))));

        // Benchmarking
        if true {
            for _ in 0..10000 {
                circuit.add_random_component()
            }

            for _ in 0..10000 {
                circuit.add_random_connection()
            }
        }

        circuit
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
        let gate = gates[rand::random::<usize>() % gates.len()];

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

        // Update element indices in selection
        let index_modified_elements: Vec<ElementIdx> = self
            .selection
            .elements
            .iter()
            .copied()
            .filter(|&elm| elm.0 > index)
            .map(|ElementIdx(index)| ElementIdx(index - 1))
            .collect();

        self.selection.elements.retain(|&elm| elm.0 < index);
        self.selection.elements.extend(index_modified_elements);

        // Finally remove the element
        self.elements.remove(index);
    }

    // Some gates will change state based on click events
    pub fn click_gate(&mut self, index: usize) {
        println!("Clicked gate {}", index);

        match &mut self.elements[index].gate {
            Gate::Button(state) => *state = true,
            Gate::Input(state) => *state = !*state,
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

    pub fn remove_connection(&mut self, idx: ConnectionIdx) {
        self.connections.remove(idx.0);
    }

    pub fn hit_test(&self, position: Vec2) -> Option<HitTestResult> {
        for (element_idx, element) in self.elements.iter().enumerate() {
            for (input_idx, offset) in element.gate.input_offsets().iter().enumerate() {
                let bounds =
                    Bounds::from_center_and_size(element.position + *offset, vec2(0.1, 0.1));

                if bounds.contains(position) {
                    return Some(HitTestResult::IO(
                        InputSpecifier(ElementIdx(element_idx), InputIdx(input_idx)).into(),
                    ));
                }
            }

            for (output_idx, offset) in once(element.gate.output_offset()).enumerate() {
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
                    .iter()
                    .enumerate()
                    .map(move |(input_idx, _)| {
                        IOSpecifier::Input(InputSpecifier(element_idx, InputIdx(input_idx)))
                    })
                    .chain(once(IOSpecifier::Output(OutputSpecifier(
                        element_idx,
                        OutputIdx(0),
                    ))))
            })
    }

    fn io_position(&self, spec: impl Into<IOSpecifier>) -> Vec2 {
        let spec = spec.into();
        let element = &self[spec.element()];
        let offset = match spec {
            IOSpecifier::Input(InputSpecifier(_, input_idx)) => {
                element.gate.input_offsets()[input_idx.0]
            }
            IOSpecifier::Output(_) => element.gate.output_offset(),
        };

        element.position + offset
    }

    pub fn handle_inputs(&mut self, input_state: &InputState, game_input: &mut GameInput) {
        let x_key = winit::keyboard::Key::Character("x".into());
        let shift_key = winit::keyboard::Key::Named(winit::keyboard::NamedKey::Shift);

        let delete_pressed = input_state.keyboard.pressed(x_key);
        let shift_down = input_state.keyboard.down(shift_key);

        let left_click = input_state.left_mouse.released && !input_state.dragging();

        let mut clear_selection = left_click;
        if shift_down {
            clear_selection = false;
        }
        let box_select = input_state.dragging() && game_input.active.is_none();

        if box_select {
            self.selection.bound_select = Some(Bounds::from_points(
                input_state.drag_start_position_world.unwrap(),
                input_state.mouse_world_position,
            ));
        } else {
            self.selection.bound_select = None;
        }

        match game_input {
            GameInput { .. } if box_select && input_state.left_mouse.released => {
                if let Some(bounds) = self.selection.bound_select {
                    self.selection.elements = self
                        .elements
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, element)| {
                            if bounds.overlaps(&element.bounds()) {
                                Some(ElementIdx(idx))
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.selection.bound_select = None;
                }
            }
            GameInput {
                active: Some(HitTestResult::Element(elm)),
                ..
            } if shift_down && left_click => {
                self.selection.toggle(*elm);
            }
            GameInput {
                active: Some(HitTestResult::Element(elm)),
                ..
            } if input_state.dragging() => {
                if self.selection.contains(*elm) {
                    for elm in self.selection.elements.iter() {
                        self.elements[elm.0].position += input_state.mouse_world_position_delta;
                    }
                } else {
                    self[*elm].position += input_state.mouse_world_position_delta;
                }
            }
            GameInput {
                hot: Some(HitTestResult::IO(IOSpecifier::Output(output))),
                active: Some(HitTestResult::IO(IOSpecifier::Input(input))),
                ..
            }
            | GameInput {
                hot: Some(HitTestResult::IO(IOSpecifier::Input(input))),
                active: Some(HitTestResult::IO(IOSpecifier::Output(output))),
            } if input_state.left_mouse.released => {
                self.add_connection(output.to(*input));
            }
            GameInput {
                hot: Some(HitTestResult::IO(spec)),
                ..
            } if delete_pressed => self.remove_connections(*spec),
            GameInput {
                hot: Some(HitTestResult::Connection(idx)),
                ..
            } if delete_pressed => {
                self.remove_connection(*idx);
            }
            GameInput {
                hot: Some(HitTestResult::Element(elm)),
                ..
            } if delete_pressed => {
                self.remove_gate(elm.0);
            }
            GameInput {
                hot: Some(HitTestResult::Element(elm)),
                ..
            } if left_click => {
                self.click_gate(elm.0);
            }
            _ => {}
        }

        if clear_selection {
            self.selection.clear()
        }
    }

    pub fn cubic_bezier_from_connection(&self, connection: &Connection) -> CubicBezier {
        let from_elm = &self[connection.from.0];
        let from = from_elm.gate.output_offset() + from_elm.position;

        let to_elm = &self[connection.to.0];
        let to = to_elm.gate.input_offsets()[connection.to.1 .0] + to_elm.position;

        CubicBezier::between_points(from, to)
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
