use crate::simulate::{simulate_function, SimulationOptions, SimulationResult};

pub struct ScanOptions<S> {
    pub num_points: usize,
    pub initial_state: S,
}

pub fn scan_function<S, P>(
    f: impl Fn(S, &P) -> S,
    d: impl Fn(&S, &S) -> f64,
    p: impl Fn(usize, usize) -> P,
    scan_options: ScanOptions<S>,
    simulation_options: SimulationOptions,
) -> Vec<SimulationResult<S>>
where
    S: Default + Copy,
{
    let mut results = Vec::with_capacity(scan_options.num_points);

    for i in 0..scan_options.num_points {
        let parameters = p(i, scan_options.num_points);
        let result = simulate_function(
            &f,
            &d,
            scan_options.initial_state,
            parameters,
            &simulation_options,
        );

        results.push(result)
    }

    results
}
