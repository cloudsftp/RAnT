use rantlib::{
    output::text::write_results,
    scan::{adapters::ParameterAdapter1DEven, generators::VectorGenerator1D, scan},
};

#[derive(Debug)]
struct Parameters {
    lambda: f64,
}

fn differential(x: f64, params: &Parameters) -> f64 {
    params.lambda * x
}

fn construct_parameters(a: f64) -> (f64, Parameters) {
    (0.5, Parameters { lambda: a })
}

fn simulate(mut x: f64, params: &Parameters) -> Vec<f64> {
    let mut result = vec![];

    let iterations = 3;
    let delta = 0.1;

    for _ in 0..iterations {
        result.push(x);
        x = x + differential(x, params) * delta;
    }

    result
}

fn project_results(_: f64, parameters: Parameters, result: Vec<f64>) -> Option<String> {
    Some(format!("{:} {:?}", parameters.lambda, result))
}

fn main() {
    let generator = VectorGenerator1D { resolution: 2 };

    let adapter = ParameterAdapter1DEven {
        start: -2.,
        end: -1.,
        construct_initial_state_and_parameters: construct_parameters,
    };

    let result = scan(generator, adapter, simulate);

    write_results(result, project_results, "orbits.tnar")
        .expect("something went wrong while writing");
}
