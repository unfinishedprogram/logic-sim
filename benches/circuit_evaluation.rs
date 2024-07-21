use glam::Vec2;
use logic_sim::logic::{
    circuit::{embedded::EmbeddedCircuit, Circuit},
    gate::Gate,
};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let mut circuit = Circuit::full_adder();
    c.bench_function("Full Adder Circuit Eval", |b| b.iter(|| circuit.step()));

    let mut circuit = Circuit::default();
    circuit.add_gate(
        Gate::Embedded(EmbeddedCircuit::new(Circuit::full_adder()).unwrap()),
        Vec2::ZERO,
    );

    c.bench_function("Full Adder Circuit Embedded", |b| b.iter(|| circuit.step()));

    let mut circuit = Circuit::adder_8_bit();
    c.bench_function("8 bit Adder Circuit", |b| b.iter(|| circuit.step()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
