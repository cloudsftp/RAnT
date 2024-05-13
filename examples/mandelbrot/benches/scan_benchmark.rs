use criterion::{criterion_group, criterion_main, Criterion};
use mandelbrot::{complex::C, mandelbrot};
use rant::scan::{
    adapters::ParameterAdapter2DEven, generators::VectorGenerator2D, scan, Simulator,
};

fn construct_parameters(x: f64, y: f64) -> (C, C) {
    (C::new(0., 0.), C::new(x, y))
}

struct DivergenceSimulator<State, Parameters> {
    max_iterations: usize,
    function: fn(State, &Parameters) -> State,
    condition: fn(&State) -> bool,
}

impl<State, Parameters> Simulator<State, Parameters> for DivergenceSimulator<State, Parameters> {
    type Result = Option<usize>;

    fn simulate(&self, initial_state: State, parameters: &Parameters) -> Self::Result {
        let mut x = initial_state;

        for i in 0..self.max_iterations {
            x = (self.function)(x, &parameters);

            if (self.condition)(&x) {
                return Some(i);
            }
        }

        None
    }
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
            let simulator = DivergenceSimulator {
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

criterion_group!(benches, scan_test);
criterion_main!(benches);
