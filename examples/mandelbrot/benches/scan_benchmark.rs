use criterion::{criterion_group, criterion_main, Criterion};
use mandelbrot::{complex::C, simulate_move, simulate_mut};
use rant::scan::{adapters::ParameterAdapter1DEven, generators::VectorGenerator1D, scan};

fn construct_parameters(x: f64) -> (C, C) {
    (C::new(0., 0.), C::new(x, 0.))
}

fn scan_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("scan");

    group.bench_function("simple scan of mandelbrot function move", |b| {
        let resolution = 1_000;

        let start = -2.;
        let end = 2.;

        b.iter(|| {
            let generator = VectorGenerator1D { resolution };
            let parameter_adapter = ParameterAdapter1DEven {
                start,
                end,
                construct_initial_state_and_parameters: construct_parameters,
            };
            let _result = scan(generator, parameter_adapter, simulate_move);
        })
    });

    group.bench_function("simple scan of mandelbrot function mut", |b| {
        let resolution = 1_000;

        let start = -2.;
        let end = 2.;

        b.iter(|| {
            let generator = VectorGenerator1D { resolution };
            let parameter_adapter = ParameterAdapter1DEven {
                start,
                end,
                construct_initial_state_and_parameters: construct_parameters,
            };
            let _result = scan(generator, parameter_adapter, simulate_mut);
        })
    });

    group.finish();
}

criterion_group!(benches, scan_test);
criterion_main!(benches);
