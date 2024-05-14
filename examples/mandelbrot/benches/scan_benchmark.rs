use criterion::{criterion_group, criterion_main, Criterion};
use mandelbrot::{complex::C, condition, mandelbrot, simulate};
use rant::{
    scan::{adapters::ParameterAdapter2DEven, generators::VectorGenerator2D, scan, scan_fn},
    simulate::condition::ConditionSimulator,
};

fn construct_parameters(x: f64, y: f64) -> (C, C) {
    (C::new(0., 0.), C::new(x, y))
}

fn scan_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("scan");

    let resolution = (100, 100);
    let start = (-2., -1.5);
    let end = (2., 1.5);

    group.bench_function("simple scan of mandelbrot function with trait", |b| {
        b.iter(|| {
            let generator = VectorGenerator2D { resolution };
            let parameter_adapter = ParameterAdapter2DEven {
                start,
                end,
                construct_initial_state_and_parameters: construct_parameters,
            };
            let simulator = ConditionSimulator {
                max_iterations: 1_000,
                function: mandelbrot,
                condition,
            };
            let result = scan(generator, parameter_adapter, simulator);
        })
    });

    group.bench_function("simple scan of mandelbrot function with function", |b| {
        b.iter(|| {
            let generator = VectorGenerator2D { resolution };
            let parameter_adapter = ParameterAdapter2DEven {
                start,
                end,
                construct_initial_state_and_parameters: construct_parameters,
            };
            let result = scan_fn(generator, parameter_adapter, simulate);
        })
    });

    group.finish();
}

criterion_group!(benches, scan_bench);
criterion_main!(benches);
