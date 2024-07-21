use glam::Vec2;

use crate::logic::gate::Gate;

use super::{
    connection::{ElementIdx, InputIdx, InputSpecifier, OutputIdx, OutputSpecifier},
    Circuit,
};

#[derive(Clone, Debug)]
pub struct EmbeddedCircuit {
    circuit: Box<Circuit>,

    output_idx: ElementIdx,
    output_count: usize,

    input_idx: ElementIdx,
    input_count: usize,
}

fn disconnected_outputs(circuit: &Circuit) -> Vec<OutputSpecifier> {
    let mut res = vec![];
    let mut output_mask: Vec<u64> = circuit
        .elements
        .iter()
        .map(|elm| (1 << elm.gate.output_count()) - 1)
        .collect();

    for conn in &circuit.connections {
        output_mask[conn.from.0 .0] &= !(1 << conn.from.1 .0);
    }

    for (index, element) in circuit.elements.iter().enumerate() {
        for output_idx in 0..element.gate.output_count() {
            let output = OutputSpecifier(ElementIdx(index), OutputIdx(output_idx));
            if output_mask[output.0 .0] & (1 << output.1 .0) != 0 {
                res.push(output);
            }
        }
    }

    res
}

fn disconnected_inputs(circuit: &Circuit) -> Vec<InputSpecifier> {
    let mut res = vec![];
    let mut input_mask: Vec<u64> = circuit
        .elements
        .iter()
        .map(|elm| (1 << elm.gate.input_count()) - 1)
        .collect();

    for conn in &circuit.connections {
        input_mask[conn.to.0 .0] &= !(1 << conn.to.1 .0);
    }

    for (index, element) in circuit.elements.iter().enumerate() {
        for input_idx in 0..element.gate.input_count() {
            let input = InputSpecifier(ElementIdx(index), InputIdx(input_idx));
            if input_mask[input.0 .0] & (1 << input.1 .0) != 0 {
                res.push(input);
            }
        }
    }

    res
}

impl EmbeddedCircuit {
    pub fn input_count(&self) -> usize {
        self.input_count
    }
    pub fn output_count(&self) -> usize {
        self.output_count
    }

    pub fn new(mut circuit: Circuit) -> Option<Self> {
        let inputs = disconnected_inputs(&circuit);
        let outputs = disconnected_outputs(&circuit);

        let input_elm = circuit.add_gate(Gate::Buf, Vec2::ZERO);
        let output_elm = circuit.add_gate(Gate::Buf, Vec2::ZERO);

        for (id, input) in inputs.iter().enumerate() {
            circuit.add_connection(OutputSpecifier(input_elm, OutputIdx(id)).to(*input));
        }

        for (id, output) in outputs.iter().enumerate() {
            circuit.add_connection(output.to(InputSpecifier(output_elm, InputIdx(id))));
        }

        Some(Self {
            circuit: Box::new(circuit),
            output_idx: output_elm,
            output_count: outputs.len(),
            input_idx: input_elm,
            input_count: inputs.len(),
        })
    }

    pub fn eval(&mut self, inputs: u64) -> u64 {
        self.circuit.right_size_solver();
        self.circuit.solver.output_results.inner[self.input_idx.0] = inputs;
        self.circuit.step();
        self.circuit.solver.output_results.inner[self.output_idx.0]
    }
}
