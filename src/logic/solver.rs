use super::{
    circuit::{
        connection::{ElementIdx, InputSpecifier, OutputSpecifier},
        Circuit,
    },
    gate::Gate,
};

#[derive(Default, Clone, Debug)]
pub struct GateIOValues {
    pub inner: Vec<u64>,
}

impl GateIOValues {
    fn new(size: usize) -> Self {
        Self {
            inner: vec![0; size],
        }
    }

    pub fn write_input(&mut self, InputSpecifier(elm, bit): InputSpecifier, value: bool) {
        if value {
            self.inner[elm.0] |= 1 << bit.0;
        } else {
            self.inner[elm.0] &= !(1 << bit.0);
        }
    }

    pub fn read_output(&self, OutputSpecifier(elm, bit): OutputSpecifier) -> bool {
        (self.inner[elm.0] >> bit.0) & 1 == 1
    }
}

#[derive(Default, Clone, Debug)]
pub struct SolverState {
    pub output_results: GateIOValues,
}

impl SolverState {
    pub fn step(mut self, circuit: &mut Circuit) -> Self {
        self.set_size(circuit.elements.len());

        let mut gate_outputs = self.output_results;
        let mut gate_inputs = GateIOValues::new(circuit.elements.len());

        for connection in &circuit.connections {
            let from = connection.from;
            let to = connection.to;
            gate_inputs.write_input(to, gate_outputs.read_output(from));
        }
        gate_outputs.inner.fill(0);
        for gate in 0..circuit.elements.len() {
            Self::eval_gate(circuit, &mut gate_outputs, &gate_inputs, ElementIdx(gate));
        }

        self.output_results = gate_outputs;

        self
    }

    #[inline(always)]
    fn eval_gate(
        circuit: &mut Circuit,
        gate_outputs: &mut GateIOValues,
        gate_inputs: &GateIOValues,
        gate: ElementIdx,
    ) {
        let inputs = &gate_inputs.inner[gate.0];
        let result = circuit[gate].gate.eval(inputs);
        gate_outputs.inner[gate.0] = result;
    }

    pub fn set_size(&mut self, size: usize) {
        if self.output_results.inner.len() == size {
            return;
        }
        self.output_results = GateIOValues::new(size);
    }
}

impl Gate {
    #[inline(always)]
    pub fn eval(&mut self, inputs: &u64) -> u64 {
        match self {
            Gate::Embedded(embed) => embed.eval(*inputs),
            Gate::Const(v) => *v as u64,
            Gate::Button(v) => {
                let res = *v as u64;
                *v = false;
                res
            }
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
            Gate::Input(_) => *inputs,
            Gate::Output(_) => 0,
        }
    }
}
