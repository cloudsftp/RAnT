#[cfg(test)]
mod tests;

use super::{DifferentialState, StateDerivative};

pub struct SimulationOptions {
    pub time_step: f64,
    pub max_time: f64,
    pub delta: f64,
}

pub fn simulate<State, Parameters, Derivative>(
    initial_state: State,
    parameters: &Parameters,
    options: SimulationOptions,
) -> Option<State>
where
    Derivative: StateDerivative + std::fmt::Debug,
    State: DifferentialState<Parameters, Derivative> + Copy + std::fmt::Debug,
{
    let mut state = initial_state;
    let mut t = 0.;

    while t < options.max_time {
        let derivative = state.derive(parameters);
        let next_state = state.step(parameters, &derivative, t);
        dbg!(t, &derivative, &next_state);

        if derivative.abs() < options.delta {
            return Some(next_state);
        }

        state = next_state;
        t += options.time_step;
    }

    None
}
