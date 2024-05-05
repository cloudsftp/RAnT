pub mod one;
mod points;

use crate::simulate::{simulate_function, SimulationOptions, SimulationResult};

use self::points::generate_scan_points;

pub struct ScanOptions {
    pub resolutions: Vec<usize>,
}

pub trait ParameterGenerator<S, P> {
    fn generate_parameters(indices: &[usize]) -> (S, P);
}

pub fn scan_function<S, P>(
    f: impl Fn(S, &P) -> S,
    dist: impl Fn(&S, &S) -> f64,
    parameter_generator: impl Fn(&[(usize, usize)]) -> (S, P),
    scan_options: ScanOptions,
    simulation_options: SimulationOptions,
) -> Vec<(S, P, SimulationResult<S>)>
where
    S: Default + Copy,
{
    let scan_points = generate_scan_points(&scan_options.resolutions);

    //let num_points_total = scan_options.resolutions.iter().map(|res| res + 1).sum();
    let mut results = Vec::with_capacity(scan_points.len());

    for scan_point in scan_points {
        let scan_point: Vec<_> = scan_point
            .into_iter()
            .zip(scan_options.resolutions.iter().copied())
            .collect();
        let (initial_state, parameters) = parameter_generator(&scan_point);

        let result = simulate_function(&f, &dist, initial_state, &parameters, &simulation_options);
        results.push((initial_state, parameters, result));
    }

    results
}
