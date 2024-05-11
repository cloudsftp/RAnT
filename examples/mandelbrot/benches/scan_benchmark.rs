use criterion::{criterion_group, criterion_main, Criterion};
use mandelbrot::{complex::C, simulate_move, simulate_mut};
use rant::scan::{adapters::ParameterAdapter2DEven, generators::VectorGenerator2D, scan};

fn construct_parameters(x: f64, y: f64) -> (C, C) {
    (C::new(0., 0.), C::new(x, y))
}

fn scan_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("scan");

    let resolution = (100, 100);
    let start = (-2., -1.5);
    let end = (2., 1.5);

    group.bench_function("simple scan of mandelbrot function move", |b| {
        b.iter(|| {
            let generator = VectorGenerator2D { resolution };
            let parameter_adapter = ParameterAdapter2DEven {
                start,
                end,
                construct_initial_state_and_parameters: construct_parameters,
            };
            let _result = scan(generator, parameter_adapter, simulate_move);
        })
    });

    group.bench_function("simple scan of mandelbrot function mut", |b| {
        b.iter(|| {
            let generator = VectorGenerator2D { resolution };
            let parameter_adapter = ParameterAdapter2DEven {
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
