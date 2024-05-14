use criterion::{criterion_group, criterion_main, Criterion};
use mandelbrot::{complex::C, mandelbrot};
use rant::{
    scan::{adapters::ParameterAdapter2DEven, generators::VectorGenerator2D, scan},
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

    let bench_function = group.bench_function("simple scan of mandelbrot function move", |b| {
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
                condition: |_| false,
            };
            let result = scan(generator, parameter_adapter, simulator);
            print!("{:?}", result.last().unwrap());
        })
    });

    group.finish();
}

criterion_group!(benches, scan_bench);
criterion_main!(benches);
