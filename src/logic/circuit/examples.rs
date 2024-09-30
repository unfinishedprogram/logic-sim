use glam::Vec2;

use crate::logic::gate::Gate;

use super::Circuit;

impl Circuit {
    pub fn full_adder() -> Self {
        let mut adder = Circuit::default();
        let in_a = adder.add_gate(Gate::Buf, Vec2::ZERO).output(0);
        let in_b = adder.add_gate(Gate::Buf, Vec2::ZERO).output(0);
        let carry = adder.add_gate(Gate::Buf, Vec2::ZERO).output(0);

        let a_xor_b = adder.add_gate(Gate::Xor, Vec2::ZERO);
        adder.add_connection(in_a.to(a_xor_b.input(0)));
        adder.add_connection(in_b.to(a_xor_b.input(1)));

        let a_and_b = adder.add_gate(Gate::And, Vec2::ZERO);
        adder.add_connection(in_a.to(a_and_b.input(0)));
        adder.add_connection(in_b.to(a_and_b.input(1)));

        let sum = adder.add_gate(Gate::Xor, Vec2::ZERO);
        adder.add_connection(a_xor_b.output(0).to(sum.input(0)));
        adder.add_connection(carry.to(sum.input(1)));

        let pre_carry_out = adder.add_gate(Gate::And, Vec2::ZERO);
        adder.add_connection(a_xor_b.output(0).to(pre_carry_out.input(0)));
        adder.add_connection(carry.to(pre_carry_out.input(1)));

        let carry_out = adder.add_gate(Gate::Or, Vec2::ZERO);
        adder.add_connection(a_and_b.output(0).to(carry_out.input(0)));
        adder.add_connection(pre_carry_out.output(0).to(carry_out.input(1)));

        adder
    }

    pub fn adder_8_bit() -> Self {
        let mut circuit = Circuit::default();
        let carry = circuit.add_gate(Gate::Const(false), Vec2::ZERO).output(0);
        let mut prev_adder = circuit.add_gate(Circuit::full_adder().embed().into(), Vec2::ZERO);
        circuit.add_connection(carry.to(prev_adder.input(2)));

        for _ in 0..7 {
            let adder = circuit.add_gate(Circuit::full_adder().embed().into(), Vec2::ZERO);
            circuit.add_connection(prev_adder.output(1).to(adder.input(2)));
            prev_adder = adder;
        }

        circuit
    }

    pub fn basic_test_circuit() -> Self {
        let mut circuit = Circuit::default();

        let a = circuit.add_gate(Gate::Buf, Vec2::new(0.0, 0.0));
        let b = circuit.add_gate(Gate::Not, Vec2::new(0.0, 1.0));

        let xor = circuit.add_gate(Gate::Xor, Vec2::new(3.0, 0.0));
        let and = circuit.add_gate(Gate::And, Vec2::new(3.0, 1.0));

        circuit.add_connection(a.output(0).to(xor.input(0)));
        circuit.add_connection(a.output(0).to(and.input(0)));

        circuit.add_connection(b.output(0).to(xor.input(1)));
        circuit.add_connection(b.output(0).to(and.input(1)));

        circuit
    }
}
