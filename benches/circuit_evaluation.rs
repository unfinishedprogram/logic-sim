use logic_sim::logic::circuit::Circuit;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let mut circuit = Circuit::basic_test_circuit();
    c.bench_function("Basic Test Circuit", |b| b.iter(|| circuit.step()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
