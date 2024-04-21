use criterion::{criterion_group, criterion_main, Criterion};
use rant::{
    scan::one::{scan_1function, scan_function, ScanOptions, ScanOptions1},
    simulate::one::SimulationOptions as SimulationOptions1,
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
const END: f64 = 4.;
const RESOLUTION: usize = 1_000;

fn parameter_generator(i: usize, len: usize) -> Parameters {
    Parameters {
        a: START + (i as f64 / len as f64) * (END - START) / RESOLUTION as f64,
    }
}

const ITERATIONS: usize = 1000;
const MAX_PERIOD: usize = 128;
const DELTA: f64 = 1e-9;

pub fn cycles_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("cycles");

    group.bench_function("cycle scan of logistic function - generic", |b| {
        b.iter(|| {
            let scan_options = ScanOptions {
                num_points: RESOLUTION,
                initial_state: 0.5,
            };

            let simulation_options = SimulationOptions {
                iterations: ITERATIONS,
                max_period: MAX_PERIOD,
                delta: DELTA,
            };

            let _ = scan_function(
                logistic,
                distance,
                parameter_generator,
                scan_options,
                simulation_options,
            );
        })
    });

    group.bench_function("cycle scan of logistic function - non-generic", |b| {
        b.iter(|| {
            let scan_options = ScanOptions1 {
                num_points: RESOLUTION,
                initial_state: 0.5,
            };

            let simulation_options = SimulationOptions1 {
                iterations: ITERATIONS,
                max_period: MAX_PERIOD,
                delta: DELTA,
            };

            let _ = scan_1function(
                logistic,
                distance,
                parameter_generator,
                scan_options,
                simulation_options,
            );
        })
    });

    group.finish();
}

criterion_group!(benches, cycles_benchmark);
criterion_main!(benches);
