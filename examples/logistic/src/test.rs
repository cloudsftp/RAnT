use anyhow::anyhow;
use rant::scan::adapters::ParameterAdapter1DEven;
use rant::simulate::simulate_function;
use rant::util::tna::{assert_equals_tna_periods, read_tna_periods_file};

use rant::{
    scan::{generators::VectorGenerator1D, scan},
    simulate::SimulationOptions,
};

use super::*;

fn unproject_initial_state_and_parameters(values: Vec<f64>) -> anyhow::Result<(f64, Parameters)> {
    if values.len() == 1 {
        Ok((0.5, Parameters { a: values[0] }))
    } else {
        Err(anyhow!("expected exactly 1 value"))
    }
}

fn simulate(initial_state: &f64, parameters: &Parameters) -> SimulationResult<f64> {
    let max_period = 128;
    let iterations = 20_000;
    let delta = 1e-9;

    let simulation_options = SimulationOptions {
        iterations,
        max_period,
        delta,
    };

    simulate_function(
        &logistic,
        &distance,
        *initial_state,
        parameters,
        &simulation_options,
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
    let rant_result = scan(generator, parameter_adapter, simulate);

    assert_equals_tna_periods(rant_result, ant_result, compare_states, compare_parameters);
}
