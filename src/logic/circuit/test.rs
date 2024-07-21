use crate::logic::{circuit::Circuit, gate::Gate};
use glam::Vec2;

#[cfg(test)]
pub mod gates {
    use super::*;

    fn test_gate(gate: Gate, inputs: &'static [bool], outputs: &'static [bool]) {
        let mut circuit = Circuit::default();
        let gate_under_test = circuit.add_gate(gate, Vec2::ZERO);
        let in_zero = circuit.add_gate(Gate::Off, Vec2::ZERO).output(0);
        let in_one = circuit.add_gate(Gate::On, Vec2::ZERO).output(0);

        for (i, &input) in inputs.iter().enumerate() {
            let source = if input { in_one } else { in_zero };
            circuit.add_connection(source.to(gate_under_test.input(i)));
        }

        circuit.step_n(10);

        for (i, &output) in outputs.iter().enumerate() {
            assert_eq!(circuit.output_value(gate_under_test.output(i)), output);
        }
    }

    #[test]
    fn on() {
        test_gate(Gate::On, &[], &[true]);
    }

    #[test]
    fn off() {
        test_gate(Gate::Off, &[], &[false]);
    }

    #[test]
    fn buf() {
        test_gate(Gate::Buf, &[false], &[false]);
        test_gate(Gate::Buf, &[true], &[true]);
    }

    #[test]
    fn not() {
        test_gate(Gate::Not, &[false], &[true]);
        test_gate(Gate::Not, &[true], &[false]);
    }

    #[test]
    fn and() {
        test_gate(Gate::And, &[false, false], &[false]);
        test_gate(Gate::And, &[false, true], &[false]);
        test_gate(Gate::And, &[true, false], &[false]);
        test_gate(Gate::And, &[true, true], &[true]);
    }

    #[test]
    fn or() {
        test_gate(Gate::Or, &[false, false], &[false]);
        test_gate(Gate::Or, &[false, true], &[true]);
        test_gate(Gate::Or, &[true, false], &[true]);
        test_gate(Gate::Or, &[true, true], &[true]);
    }

    #[test]
    fn xor() {
        test_gate(Gate::Xor, &[false, false], &[false]);
        test_gate(Gate::Xor, &[false, true], &[true]);
        test_gate(Gate::Xor, &[true, false], &[true]);
        test_gate(Gate::Xor, &[true, true], &[false]);
    }

    #[test]
    fn nand() {
        test_gate(Gate::Nand, &[false, false], &[true]);
        test_gate(Gate::Nand, &[false, true], &[true]);
        test_gate(Gate::Nand, &[true, false], &[true]);
        test_gate(Gate::Nand, &[true, true], &[false]);
    }

    #[test]
    fn nor() {
        test_gate(Gate::Nor, &[false, false], &[true]);
        test_gate(Gate::Nor, &[false, true], &[false]);
        test_gate(Gate::Nor, &[true, false], &[false]);
        test_gate(Gate::Nor, &[true, true], &[false]);
    }

    #[test]
    fn xnor() {
        test_gate(Gate::Xnor, &[false, false], &[true]);
        test_gate(Gate::Xnor, &[false, true], &[false]);
        test_gate(Gate::Xnor, &[true, false], &[false]);
        test_gate(Gate::Xnor, &[true, true], &[true]);
    }
}

#[test]
fn full_adder() {
    fn make_adder(in_a: bool, in_b: bool, carry: bool) -> (bool, bool) {
        let mut adder = Circuit::default();
        let in_a = adder.add_gate(Gate::Input(in_a), Vec2::ZERO).output(0);
        let in_b = adder.add_gate(Gate::Input(in_b), Vec2::ZERO).output(0);
        let carry = adder.add_gate(Gate::Input(carry), Vec2::ZERO).output(0);

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

        adder.step_n(4);

        (
            adder.output_value(sum.output(0)),
            adder.output_value(carry_out.output(0)),
        )
    }

    assert_eq!(make_adder(false, false, false), (false, false));
    assert_eq!(make_adder(false, false, true), (true, false));
    assert_eq!(make_adder(false, true, false), (true, false));
    assert_eq!(make_adder(false, true, true), (false, true));
    assert_eq!(make_adder(true, false, false), (true, false));
    assert_eq!(make_adder(true, false, true), (false, true));
    assert_eq!(make_adder(true, true, false), (false, true));
    assert_eq!(make_adder(true, true, true), (true, true));
}
