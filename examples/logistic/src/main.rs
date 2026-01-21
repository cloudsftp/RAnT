#[cfg(test)]
mod test;

use rantlib::{
    output::text::write_results,
    scan::{adapters::ParameterAdapter1DEven, generators::VectorGenerator1D, scan},
    simulate::period::{simulate, Cycle, SimulationOptions},
};

use std::{cmp::Ordering, fmt::format};

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

fn construct_parameters(a: f64) -> (f64, Parameters) {
    (0.5, Parameters { a })
}

fn project_results_period(_: f64, parameters: Parameters, result: Cycle<f64>) -> Option<String> {
    let period = match &result {
        Cycle::FixedPoint(_) => 1,
        Cycle::Cycle(cycle) => cycle.len(),
        Cycle::Divergence => 0,
    };

    Some(format!("{:} {}", parameters.a, period))
}

fn project_results_limit_object(
    _: f64,
    parameters: Parameters,
    result: Cycle<f64>,
) -> Option<String> {
    let object = match &result {
        Cycle::FixedPoint(x) => format!("(1) {x}"),
        Cycle::Cycle(cycle) => format!("({}) {:?}", cycle.len(), cycle),
        Cycle::Divergence => format!("diverged"),
    };

    Some(format!("{}: {}", parameters.a, object))
}

const MAX_PERIOD: usize = 128;
const ITERATIONS: usize = 20_000;
const DELTA: f64 = 1e-9;

fn simulate_logistic(x: f64, parameters: &Parameters) -> Cycle<f64> {
    simulate(
        x,
        parameters,
        logistic,
        distance,
        SimulationOptions {
            iterations: ITERATIONS,
            max_period: MAX_PERIOD,
            delta: DELTA,
        },
    )
}

const START: f64 = 3.;
const END: f64 = 4.;
const RESOLUTION: usize = 100;

fn main() {
    let vector_generator = VectorGenerator1D {
        resolution: RESOLUTION,
    };

    let parameter_adapter = ParameterAdapter1DEven {
        start: START,
        end: END,
        construct_initial_state_and_parameters: |a| (0.5, Parameters { a }),
    };

    let result = scan(vector_generator, parameter_adapter, simulate_logistic);

    write_results(result, project_results_limit_object, "period.tnar")
        .expect("something went wrong while writing");
}
