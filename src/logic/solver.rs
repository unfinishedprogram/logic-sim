use super::{
    circuit::{
        connection::{ElementIdx, InputSpecifier, OutputSpecifier},
        Circuit,
    },
    gate::Gate,
};

#[derive(Default, Clone)]
pub struct GateIOValues {
    inner: Vec<u64>,
}

impl GateIOValues {
    fn new(size: usize) -> Self {
        Self {
            inner: vec![0; size],
        }
    }

    fn write_input(&mut self, InputSpecifier(elm, bit): InputSpecifier, value: bool) {
        if value {
            self.inner[elm.0] |= 1 << bit.0;
        } else {
            self.inner[elm.0] &= !(1 << bit.0);
        }
    }

    pub fn read_output(&self, OutputSpecifier(elm, bit): OutputSpecifier) -> bool {
        self.inner[elm.0] >> bit.0 & 1 == 1
    }
}

#[derive(Default, Clone)]
pub struct SolverState {
    previous_results: GateIOValues,
    pub output_results: GateIOValues,
}

impl SolverState {
    pub fn step(mut self, circuit: &Circuit) -> Self {
        if circuit.elements.len() != self.output_results.inner.len() {
            self.output_results = GateIOValues::new(circuit.elements.len());
            self.previous_results = GateIOValues::new(circuit.elements.len());
        }

        self.previous_results = self.output_results;
        self.output_results = GateIOValues::new(circuit.elements.len());
        let mut gate_inputs = GateIOValues::new(circuit.elements.len());

        for connection in &circuit.connections {
            let from = connection.from;
            let to = connection.to;
            gate_inputs.write_input(to, self.previous_results.read_output(from));
        }

        for gate in 0..circuit.elements.len() {
            self.eval_gate(circuit, &gate_inputs, ElementIdx(gate));
        }

        self
    }

    fn eval_gate(&mut self, circuit: &Circuit, gate_inputs: &GateIOValues, gate: ElementIdx) {
        let inputs = &gate_inputs.inner[gate.0];
        let result = circuit[gate].gate.eval(inputs);
        self.output_results.inner[gate.0] = result;
    }
}

impl Gate {
    pub fn eval(&self, inputs: &u64) -> u64 {
        match self {
            Gate::Input(v) => *v as u64,
            Gate::Button(v) => *v as u64,
            Gate::And => 1 & (inputs & (inputs >> 1)),
            Gate::Or => 1 & (inputs | (inputs >> 1)),
            Gate::Not => 1 & (!inputs),
            Gate::Buf => *inputs,
            Gate::Xor => 1 & (inputs ^ (inputs >> 1)),
            Gate::Nand => !(1 & (inputs & (inputs >> 1))),
            Gate::Nor => !(1 & (inputs | (inputs >> 1))),
            Gate::Xnor => 1 & (!(inputs ^ (inputs >> 1))),
            Gate::On => 1,
            Gate::Off => 0,
        }
    }
}
