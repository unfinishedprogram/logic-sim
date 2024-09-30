use std::collections::{HashMap, HashSet};

use glam::{vec2, Vec2};

use crate::game::{input::InputState, GameInput, PrevGameInput};

use util::bounds::Bounds;

use super::connection::{
    ConnectionIdx, ElementIdx, IOSpecifier, InputIdx, InputSpecifier, OutputIdx, OutputSpecifier,
};

use crate::logic::hit_test::HitTestResult;

use super::Circuit;

#[derive(Default)]
pub struct ElementSelection {
    pub(super) elements: HashSet<HitTestResult>,
    pub(super) bound_select: Option<Bounds>,
}

#[derive(Default)]
pub struct EditCircuit {
    pub(crate) circuit: Circuit,
    pub(crate) selection: ElementSelection,
    pub(crate) clipboard: Option<Circuit>,
}

impl EditCircuit {
    fn remove_elements(&mut self, elements: ElementSelection) {
        // Selection deletion must be applied as a single batch
        // Since deleting single circuit elements invalidates connection pointers, the order of deletion matters
        // 1. Delete individual connections
        // 2. Delete connections to/from selected input/output nodes
        // 3. Delete selected circuit elements

        // 1. Delete individual connections
        // Since we delete them all in a single batch, we don't need to worry about invalidating indices
        self.circuit
            .remove_many_connections(elements.connections().into_iter().collect::<HashSet<_>>());

        // 2. Delete connections to/from selected input/output nodes
        for node in elements.connection_nodes() {
            self.circuit.remove_connections(node);
        }

        // 3. Delete selected circuit elements
        // We must remove the elements in reverse index order to avoid invalidating indices
        let mut gates: Vec<_> = elements.elements().into_iter().collect();
        gates.sort_unstable_by_key(|v| v.0);
        for element in gates.iter().rev() {
            self.circuit.remove_gate(*element);
        }
    }

    fn extract_elements_into_circuit(&mut self, elements: ElementSelection) -> Circuit {
        let mut res = Circuit::default();

        // First create a lookup table of remapped element IDs in the new circuit
        let mut circuit_indexes = HashMap::<ElementIdx, ElementIdx>::new();
        for gate_idx in elements.elements() {
            let element = &self.circuit[gate_idx];
            circuit_indexes.insert(
                gate_idx,
                res.add_gate(element.gate.clone(), element.position),
            );
        }

        // Then we use this lookup table to add the remapped connections
        for connection_idx in elements.connections() {
            let connection = self.circuit[connection_idx];

            let Some(from) = circuit_indexes
                .get(&connection.from.0)
                .map(|new_elm_idx| OutputSpecifier(*new_elm_idx, connection.from.1))
            else {
                continue;
            };

            let Some(to) = circuit_indexes
                .get(&connection.to.0)
                .map(|new_elm_idx| InputSpecifier(*new_elm_idx, connection.to.1))
            else {
                continue;
            };

            res.add_connection(from.to(to));
        }

        res
    }

    fn paste_circuit(&mut self, mut circuit: Circuit) {
        let elements_len = self.circuit.elements.len();

        for connection in circuit.connections.iter_mut() {
            connection.from.0 .0 += elements_len;
            connection.to.0 .0 += elements_len;
        }

        self.circuit.elements.extend(circuit.elements);
        self.circuit.connections.extend(circuit.connections);

        self.selection.clear();

        for idx in elements_len..self.circuit.elements.len() {
            self.selection
                .elements
                .insert(HitTestResult::Element(ElementIdx(idx)));
        }
    }

    pub fn hit_test_bounds(&self, bounds: Bounds) -> HashSet<HitTestResult> {
        let mut res = vec![];

        for (element_idx, element) in self.circuit.elements.iter().enumerate() {
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

        for (element_idx, element) in self.circuit.elements.iter().enumerate() {
            if element.bounds().overlaps(&bounds) {
                res.push(HitTestResult::Element(ElementIdx(element_idx)))
            }
        }

        for (connection_idx, connection) in self.circuit.connections.iter().enumerate() {
            if self
                .circuit
                .cubic_bezier_from_connection(connection)
                .hit_test_bounds(bounds, 0.05)
            {
                res.push(HitTestResult::Connection(ConnectionIdx(connection_idx)))
            }
        }

        HashSet::from_iter(res)
    }

    pub fn hit_test(&self, position: Vec2) -> Option<HitTestResult> {
        for (element_idx, element) in self.circuit.elements.iter().enumerate() {
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

        for (element_idx, element) in self.circuit.elements.iter().enumerate() {
            if element.hit_test(position) {
                return Some(HitTestResult::Element(ElementIdx(element_idx)));
            }
        }

        for (connection_idx, connection) in self.circuit.connections.iter().enumerate() {
            if self
                .circuit
                .cubic_bezier_from_connection(connection)
                .hit_test(position, 0.05)
            {
                return Some(HitTestResult::Connection(ConnectionIdx(connection_idx)));
            }
        }

        None
    }

    pub fn take_selection(&mut self) -> ElementSelection {
        let mut selection = ElementSelection::default();
        // Replace the existing selection, since after deletion the indices will be invalid
        std::mem::swap(&mut self.selection, &mut selection);
        selection
    }

    pub fn handle_inputs(&mut self, input_state: &InputState, game_input: &mut GameInput) {
        let x_key = winit::keyboard::Key::Character("x".into());
        let c_key = winit::keyboard::Key::Character("c".into());
        let v_key = winit::keyboard::Key::Character("v".into());

        let shift_key = winit::keyboard::Key::Named(winit::keyboard::NamedKey::Shift);

        let delete_pressed = input_state.keyboard.pressed(x_key);
        let copy_pressed = input_state.keyboard.pressed(c_key);
        let paste_pressed = input_state.keyboard.pressed(v_key);

        let shift_down = input_state.keyboard.down(shift_key);

        let left_click = input_state.left_mouse.released && !input_state.dragging();

        let mut clear_selection = left_click;
        if shift_down {
            clear_selection = false;
        }

        let box_select = input_state.dragging()
            && game_input.active.is_none()
            && game_input.prev.active.is_none();

        if box_select {
            self.selection.bound_select = Some(Bounds::from_points(
                input_state.drag_start_position_world.unwrap(),
                input_state.mouse_world_position,
            ));
        } else {
            self.selection.bound_select = None;
        }

        match game_input {
            GameInput {
                hot: Some(HitTestResult::IO(a)),
                prev:
                    PrevGameInput {
                        active: Some(HitTestResult::IO(b)),
                        ..
                    },
                ..
            } if input_state.left_mouse.released => match (a, b) {
                (IOSpecifier::Input(input), IOSpecifier::Output(output))
                | (IOSpecifier::Output(output), IOSpecifier::Input(input)) => {
                    self.circuit.add_connection(output.to(*input));
                }
                _ => {}
            },

            GameInput { .. } if box_select && input_state.left_mouse.released => {
                if let Some(bounds) = self.selection.bound_select {
                    self.selection.elements = self.hit_test_bounds(bounds);
                    self.selection.bound_select = None;
                }
            }
            GameInput { hot: Some(res), .. } if shift_down && left_click => {
                self.selection.toggle(*res);
                println!("Toggling Selection");
            }
            GameInput {
                active: Some(elm), ..
            } if input_state.dragging() => {
                if self.selection.contains(*elm) {
                    for item in self.selection.elements.iter() {
                        match item {
                            HitTestResult::Element(elm) => {
                                self.circuit.elements[elm.0].position +=
                                    input_state.mouse_world_position_delta
                            }
                            HitTestResult::IO(_) => {}
                            HitTestResult::Connection(_) => {}
                        }
                    }
                } else if let HitTestResult::Element(elm) = elm {
                    self.circuit[*elm].position += input_state.mouse_world_position_delta;
                }
            }
            GameInput { .. } if delete_pressed => {
                let selection = self.take_selection();
                self.remove_elements(selection);
            }
            GameInput { .. } if copy_pressed => {
                let selection = self.take_selection();
                self.clipboard = Some(self.extract_elements_into_circuit(selection));
            }
            GameInput { .. } if paste_pressed => {
                if let Some(clipboard) = self.clipboard.take() {
                    self.paste_circuit(clipboard);
                }
            }
            GameInput {
                active: Some(res), ..
            } if left_click => {
                self.selection.clear();
                self.selection.toggle(*res);
            }
            _ => {}
        }

        if !input_state.left_mouse.down {
            self.selection.bound_select = None;
        }

        if clear_selection {
            self.selection.clear()
        }
    }
}

impl From<Circuit> for EditCircuit {
    fn from(value: Circuit) -> Self {
        Self {
            circuit: value,
            selection: ElementSelection::default(),
            clipboard: None,
        }
    }
}

impl ElementSelection {
    pub fn contains(&self, element: HitTestResult) -> bool {
        self.elements.contains(&element)
    }

    pub fn toggle(&mut self, element: HitTestResult) {
        if self.elements.contains(&element) {
            self.elements.remove(&element);
        } else {
            self.elements.insert(element);
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn connections(&self) -> impl IntoIterator<Item = ConnectionIdx> + '_ {
        self.elements.iter().filter_map(|hit| {
            if let HitTestResult::Connection(idx) = hit {
                Some(*idx)
            } else {
                None
            }
        })
    }

    pub fn connection_nodes(&self) -> impl IntoIterator<Item = IOSpecifier> + '_ {
        self.elements.iter().filter_map(|hit| {
            if let HitTestResult::IO(node) = hit {
                Some(*node)
            } else {
                None
            }
        })
    }

    pub fn elements(&self) -> impl IntoIterator<Item = ElementIdx> + '_ {
        self.elements.iter().filter_map(|hit| {
            if let HitTestResult::Element(element) = hit {
                Some(*element)
            } else {
                None
            }
        })
    }
}
