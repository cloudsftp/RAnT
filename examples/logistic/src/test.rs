use anyhow::anyhow;

use rantlib::{
    scan::{adapters::ParameterAdapter1DEven, generators::VectorGenerator1D, scan},
    simulate::period::{self, SimulationOptions},
    util::tna::{assert_equals_tna_periods, read_tna_periods_file},
};

use super::*;

fn unproject_initial_state_and_parameters(values: Vec<f64>) -> anyhow::Result<(f64, Parameters)> {
    if values.len() == 1 {
        Ok((0.5, Parameters { a: values[0] }))
    } else {
        Err(anyhow!("expected exactly 1 value"))
    }
}

fn simulate_logistic(initial_state: f64, parameters: &Parameters) -> Cycle<f64> {
    let max_period = 128;
    let iterations = 20_000;
    let delta = 1e-9;

    let simulation_options = SimulationOptions {
        iterations,
        max_period,
        delta,
    };

    period::simulate(
        initial_state,
        parameters,
        logistic,
        distance,
        simulation_options,
    )
}

#[test]
fn period_test() {
    let ant_result = read_tna_periods_file(
        "ant/test_data/period.tna",
        unproject_initial_state_and_parameters,
    )
    .expect("problem while reading ant/test_data/period.tna");

    let generator = VectorGenerator1D {
        resolution: RESOLUTION,
    };

    let start = 3.;
    let end = 4.;

    let parameter_adapter = ParameterAdapter1DEven {
        start,
        end,
        construct_initial_state_and_parameters: construct_parameters,
    };
    let rant_result = scan(generator, parameter_adapter, simulate_logistic).collect();

    assert_equals_tna_periods(rant_result, ant_result, compare_states, compare_parameters);
}
