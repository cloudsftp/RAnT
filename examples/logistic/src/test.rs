use anyhow::anyhow;
use rant::util::tna::{assert_equals_tna_periods, read_tna_periods_file};

use super::*;

fn unproject_initial_state_and_parameters(values: Vec<f64>) -> anyhow::Result<(f64, Parameters)> {
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
    let delta = 1e-9;

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
