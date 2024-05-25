use std::{
    fs::File,
    io::{BufWriter, Write},
    thread,
};

use criterion::{criterion_group, criterion_main, Criterion};
use mandelbrot::{complex::C, simulate_mandelbrot};
use rant::scan::{
    adapters::ParameterAdapter2DEven,
    generators::{ParallelVectorGenerator2D, VectorGenerator2D},
    scan, scan_parallel, ParallelVectorGenerator,
};
use rayon::iter::ParallelIterator;

fn construct_parameters(x: f64, y: f64) -> (C, C) {
    (C::new(0., 0.), C::new(x, y))
}

fn project_result((_, parameter, result): (C, C, Option<usize>)) -> Vec<u8> {
    format!("{:?} - {:?}\n", result, parameter).into_bytes()
}

fn scan_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("scan");

    let resolution = (1_000, 1_000);
    /* Interesting area to look at
    let start = (-0.24, 0.9);
    let end = (0.05, 1.15);
    */

    // In this area, the maximum number of iterations will be done
    let start = (-0.1, 0.1);
    let end = (-0.1, 0.1);

    group.bench_function("simple scan of mandelbrot function with function", |b| {
        b.iter(|| {
            let generator = VectorGenerator2D { resolution };
            let parameter_adapter = ParameterAdapter2DEven {
                start,
                end,
                construct_initial_state_and_parameters: construct_parameters,
            };
            let results = scan(generator, parameter_adapter, simulate_mandelbrot);

            let out_file = File::create("benches/output/mandelbrot_single_thread.tnar").unwrap();
            let mut out_file = BufWriter::new(out_file);
            for result in results {
                out_file.write_all(&project_result(result)).unwrap();
            }
            out_file.flush().unwrap();
        })
    });

    group.bench_function("parallel scan of mandelbrot function", |b| {
        b.iter(|| {
            let parallel_generator = ParallelVectorGenerator2D { resolution };
            let parameter_adapter = ParameterAdapter2DEven {
                start,
                end,
                construct_initial_state_and_parameters: construct_parameters,
            };
            let results = scan_parallel(
                parallel_generator.clone(),
                parameter_adapter,
                simulate_mandelbrot,
            );
            let num_results = parallel_generator.size_hint();

            let (sender, receiver) = crossbeam_channel::unbounded::<(C, C, Option<usize>)>();

            let out_file = File::create("benches/output/mandelbrot_multi_thread.tnar").unwrap();
            let mut out_file = BufWriter::new(out_file);
            let writer_thread = thread::spawn(move || {
                for result in receiver.iter().take(num_results) {
                    out_file.write_all(&project_result(result)).unwrap();
                }

                out_file.flush().unwrap();
            });

            results.for_each_with(sender, |sender, result| {
                sender.send(result).unwrap();
            });

            writer_thread.join().unwrap();
        })
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = scan_bench
);
criterion_main!(benches);
