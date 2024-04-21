use crate::simulate::one::{simulate_function, SimulationOptions, SimulationResult};

pub struct ScanOptions {
    pub num_points: usize,
    pub initial_state: f64,
}

pub fn scan_1function<P>(
    f: impl Fn(f64, &P) -> f64,
    p: impl Fn(usize, usize) -> P,
    scan_options: ScanOptions,
    simulation_options: SimulationOptions,
) -> Vec<SimulationResult> {
    let mut results = Vec::with_capacity(scan_options.num_points);

    for i in 0..scan_options.num_points {
        let parameters = p(i, scan_options.num_points);
        let result = simulate_function(
            &f,
            scan_options.initial_state,
            parameters,
            &simulation_options,
        );

        results.push(result)
    }

    results
}
