#[cfg(test)]
mod test;

use rant::{
    output::text::write_results,
    scan::{scan_function, ScanOptions},
    simulate::{Cycle, SimulationOptions, SimulationResult},
};

use std::cmp::Ordering;

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

fn project_results(
    _: &f64,
    parameters: &Parameters,
    result: &SimulationResult<f64>,
) -> Option<String> {
    let period = match &result.cycle {
        Cycle::FixedPoint(_) => 1,
        Cycle::Cycle(cycle) => cycle.len(),
        Cycle::Divergence => 0,
    };

    Some(format!("{:} {}", parameters.a, period))
}

fn main() {
    let max_period = 128;
    let iterations = 20_000;
    let delta = 1e-9;

    let scan_options = ScanOptions {
        resolutions: vec![RESOLUTION],
    };

    let simulation_options = SimulationOptions {
        iterations,
        max_period,
        delta,
    };

    let result = scan_function(
        logistic,
        distance,
        param_gen,
        scan_options,
        simulation_options,
    );

    write_results(result, project_results, "period.tnar")
        .expect("something went wrong while writing");
}
