use std::cmp::Ordering;

use rant::{
    scan::one::{scan_function, ScanOptions},
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
fn gen_parameters(x: usize, resolution: usize) -> Parameters {
    Parameters {
        a: START + (x as f64 / resolution as f64) * (STOP - START),
    }
}

fn main() {
    let scan_options = ScanOptions {
        resolution: RESOLUTION,
        initial_state: 0.5,
    };

    let simulation_options = SimulationOptions {
        iterations: 20_000,
        max_period: 128,
        delta: 1e-9,
    };

    let results = scan_function(
        logistic,
        distance,
        gen_parameters,
        scan_options,
        simulation_options,
    );

    println!("{:?}", results)
}

#[cfg(test)]
mod test {
    use anyhow::anyhow;
    use rant::util::tna::{assert_equals_tna_periods, read_tna_periods_file};

    use super::*;

    fn unproject_initial_state_and_parameters(
        values: Vec<f64>,
    ) -> anyhow::Result<(f64, Parameters)> {
        if values.len() == 1 {
            Ok((0.5, Parameters { a: values[0] }))
        } else {
            Err(anyhow!("expected exactly 1 value"))
        }
    }

    #[test]
    fn period_test() {
        let max_period = 128;
        let iterations = 20_000;
        let delta = 1e-24;

        let ant_result = read_tna_periods_file(
            "ant/test_data/period.tna",
            unproject_initial_state_and_parameters,
        )
        .expect("problem while reading ant/test_data/period.tna");

        let scan_options = ScanOptions {
            resolution: RESOLUTION,
            initial_state: 0.5,
        };

        let simulation_options = SimulationOptions {
            iterations,
            max_period,
            delta,
        };

        let rant_result = scan_function(
            logistic,
            distance,
            gen_parameters,
            scan_options,
            simulation_options,
        );

        assert_equals_tna_periods(rant_result, ant_result, compare_states, compare_parameters);
    }
}
