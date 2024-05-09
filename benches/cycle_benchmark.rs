use criterion::{criterion_group, criterion_main, Criterion};
use rant::{
    scan::{scan_function, ScanOptions},
    simulate::SimulationOptions,
};

struct Parameters {
    a: f64,
}

fn logistic(x: f64, parameters: &Parameters) -> f64 {
    parameters.a * x * (1. - x)
}

fn distance(a: &f64, b: &f64) -> f64 {
    (a - b).abs()
}

const START: f64 = 3.;
const STOP: f64 = 4.;
const RESOLUTION: usize = 100;
fn param_gen(scan_point: &[(usize, usize)]) -> (f64, Parameters) {
    let (x, resolution) = *scan_point.first().expect("one dimensional scan");
    (
        0.5,
        Parameters {
            a: START + (x as f64 / resolution as f64) * (STOP - START),
        },
    )
}

const ITERATIONS: usize = 1000;
const MAX_PERIOD: usize = 128;
const DELTA: f64 = 1e-9;

pub fn cycles_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("cycles");

    group.bench_function("cycle scan of logistic function", |b| {
        b.iter(|| {
            let scan_options = ScanOptions {
                resolutions: vec![RESOLUTION],
            };

            let simulation_options = SimulationOptions {
                iterations: ITERATIONS,
                max_period: MAX_PERIOD,
                delta: DELTA,
            };

            let _ = scan_function(
                logistic,
                distance,
                param_gen,
                scan_options,
                simulation_options,
            );
        })
    });

    group.finish();
}

criterion_group!(benches, cycles_benchmark);
criterion_main!(benches);
