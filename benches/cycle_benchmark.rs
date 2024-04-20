use criterion::{criterion_group, criterion_main, Criterion};
use rant::simulate::find_cycle;

fn logistic_a3(x: f64) -> f64 {
    2. * x * (1. - x)
}

const START: f64 = 0.;
const END: f64 = 1.;
const RESOLUTION: usize = 10_000;

const MAX_ITERATIONS: usize = 10_000;
const MAX_CYCLE_LENGTH: usize = 1_000;

pub fn cycles_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("cycles");
    group.bench_function("cycles of logistic function", |b| {
        b.iter(|| {
            for i in 0..RESOLUTION {
                let _ = find_cycle(
                    logistic_a3,
                    START + (END - START) / (RESOLUTION * i) as f64,
                    MAX_ITERATIONS,
                    MAX_CYCLE_LENGTH,
                );
            }
        })
    });

    group.finish();
}

criterion_group!(benches, cycles_benchmark);
criterion_main!(benches);
