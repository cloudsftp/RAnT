use std::thread;

use mandelbrot::{complex::C, simulate_mandelbrot};
use plotters::prelude::*;
use rantlib::scan::{
    adapters::ParameterAdapter2DEven, generators::ParallelVectorGenerator2D, scan_parallel,
    ParallelVectorGenerator,
};
use rayon::prelude::*;

fn construct_parameters(x: f64, y: f64) -> (C, C) {
    (C::new(0., 0.), C::new(x, y))
}

const OUT_FILE_NAME: &str = "benches/output/mandelbrot.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resolution = (1_000usize, 1_000usize);

    let start = (-0.25, 0.8);
    let end = (0.03, 1.15);

    let root = BitMapBackend::new(
        OUT_FILE_NAME,
        (resolution.0 as u32 + 50, resolution.1 as u32 + 50),
    )
    .into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(start.0..end.0, start.1..end.1)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let parallel_generator = ParallelVectorGenerator2D { resolution };
    let num_results = parallel_generator.size_hint();

    let (sender, receiver) = crossbeam_channel::unbounded::<(C, C, Option<usize>)>();

    let computing_thread = thread::spawn(move || {
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

        results.for_each_with(sender, |sender, result| {
            sender.send(result).unwrap();
        });
    });

    for (_, parameter, result) in receiver.iter().take(num_results) {
        if let Some(result) = result {
            plotting_area.draw_pixel(
                (parameter.a, parameter.b),
                &MandelbrotHSL::get_color(result as f64 / 200.),
            )?;
        } else {
            plotting_area.draw_pixel((parameter.a, parameter.b), &BLACK)?;
        }
    }

    computing_thread.join().unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
