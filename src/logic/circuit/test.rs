use crate::logic::{
    circuit::{embedded::EmbeddedCircuit, Circuit},
    gate::Gate,
};
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
    fn make_embedded_adder(in_a: bool, in_b: bool, carry: bool) -> (bool, bool) {
        let adder_gate = EmbeddedCircuit::new(Circuit::full_adder()).unwrap();
        let mut circuit = Circuit::default();
        let adder_instance = circuit.add_gate(Gate::Embedded(adder_gate), Vec2::ZERO);

        let in_a = circuit.add_gate(Gate::Const(in_a), Vec2::ZERO).output(0);
        let in_b = circuit.add_gate(Gate::Const(in_b), Vec2::ZERO).output(0);
        let carry = circuit.add_gate(Gate::Const(carry), Vec2::ZERO).output(0);

        circuit.add_connection(in_a.to(adder_instance.input(0)));
        circuit.add_connection(in_b.to(adder_instance.input(1)));
        circuit.add_connection(carry.to(adder_instance.input(2)));

        circuit.step_n(6);

        (
            circuit.output_value(adder_instance.output(0)),
            circuit.output_value(adder_instance.output(1)),
        )
    }

    assert_eq!(make_embedded_adder(false, false, false), (false, false));
    assert_eq!(make_embedded_adder(false, false, true), (true, false));
    assert_eq!(make_embedded_adder(false, true, false), (true, false));
    assert_eq!(make_embedded_adder(false, true, true), (false, true));
    assert_eq!(make_embedded_adder(true, false, false), (true, false));
    assert_eq!(make_embedded_adder(true, false, true), (false, true));
    assert_eq!(make_embedded_adder(true, true, false), (false, true));
    assert_eq!(make_embedded_adder(true, true, true), (true, true));
}
