use criterion::{criterion_group, criterion_main, Criterion};

pub fn cycles_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("cycles");

    group.bench_function("cycle scan of logistic function", |b| {
        b.iter(|| 1 + 1);
        /*
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
        */
    });

    group.finish();
}

criterion_group!(benches, cycles_benchmark);
criterion_main!(benches);
