use super::{
    circuit::{
        self,
        connection::{ElementIdx, IOSpecifier},
        Circuit,
    },
    gate::Gate,
};

#[derive(Default, Clone)]
pub struct SolverState {
    previous_results: Vec<bool>,
    pub output_results: Vec<bool>,
}

impl SolverState {
    pub fn step(mut self, circuit: &Circuit) -> Self {
        if circuit.elements.len() != self.output_results.len() {
            self.output_results = vec![false; circuit.elements.len()];
            self.previous_results = vec![false; circuit.elements.len()];
        }

        self.previous_results = self.output_results;
        self.output_results = vec![false; self.previous_results.len()];

        let mut gate_inputs = vec![vec![false; 2]; circuit.elements.len()];

        for connection in &circuit.connections {
            let from = connection.from.0;
            let to = connection.to.0;
            gate_inputs[to.0][connection.to.1 .0] |= self.previous_results[from.0];
        }

        for gate in 0..self.output_results.len() {
            self.eval_gate(circuit, &gate_inputs, ElementIdx(gate));
        }

        self
    }

    fn eval_gate(&mut self, circuit: &Circuit, gate_inputs: &[Vec<bool>], gate: ElementIdx) {
        let inputs = &gate_inputs[gate.0];
        let result = circuit[gate].gate.eval(inputs);
        self.output_results[gate.0] = result;
    }
}

impl Gate {
    pub fn eval(&self, inputs: &[bool]) -> bool {
        match self {
            Gate::Input(v) => *v,
            Gate::And => inputs[0] && inputs[1],
            Gate::Or => inputs[0] || inputs[1],
            Gate::Not => !inputs[0],
            Gate::Buf => inputs[0],
            Gate::Xor => inputs[0] != inputs[1],
            Gate::Nand => !(inputs[0] && inputs[1]),
            Gate::Nor => !(inputs[0] || inputs[1]),
            Gate::Xnor => inputs[0] == inputs[1],
        }
    }
}

pub fn get_circuit_io(circuit: &circuit::Circuit) -> Vec<IOSpecifier> {
    circuit
        .connection_dots()
        .filter(|dot| {
            circuit.connections.iter().any(|c| match dot {
                IOSpecifier::Input(input) => c.to == *input,
                IOSpecifier::Output(output) => c.from == *output,
            })
        })
        .collect()
}
