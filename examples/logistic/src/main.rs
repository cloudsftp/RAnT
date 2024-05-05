#[cfg(test)]
mod test;

use std::cmp::Ordering;

use rant::{
    scan::{scan_function, ScanOptions},
    simulate::SimulationOptions,
};

#[derive(Debug)]
struct Parameters {
    a: f64,
}

fn compare_parameters(a: &Parameters, b: &Parameters) -> Ordering {
    a.a.total_cmp(&b.a)
}

fn logistic(x: f64, params: &Parameters) -> f64 {
    params.a * x * (1. - x)
}

fn distance(a: &f64, b: &f64) -> f64 {
    (a - b).abs()
}

fn compare_states(a: &f64, b: &f64) -> Ordering {
    a.total_cmp(b)
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

fn main() {
    let scan_options = ScanOptions {
        resolutions: vec![RESOLUTION],
    };

    let simulation_options = SimulationOptions {
        iterations: 20_000,
        max_period: 128,
        delta: 1e-9,
    };

    let results = scan_function(
        logistic,
        distance,
        param_gen,
        scan_options,
        simulation_options,
    );

    println!("{:?}", results)
}
