use crate::simulate::{simulate_function, SimulationOptions, SimulationResult};

pub struct ScanOptions<S> {
    pub resolution: usize,
    pub initial_state: S,
}

pub fn scan_function<S, P>(
    f: impl Fn(S, &P) -> S,
    dist: impl Fn(&S, &S) -> f64,
    param_gen: impl Fn(usize, usize) -> P,
    scan_options: ScanOptions<S>,
    simulation_options: SimulationOptions,
) -> Vec<(S, P, SimulationResult<S>)>
where
    S: Default + Copy,
{
    let num_points = scan_options.resolution + 1;
    let mut results = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let parameters = param_gen(i, scan_options.resolution);
        let result = simulate_function(
            &f,
            &dist,
            scan_options.initial_state,
            &parameters,
            &simulation_options,
        );

        results.push((scan_options.initial_state, parameters, result));
    }

    results
}
