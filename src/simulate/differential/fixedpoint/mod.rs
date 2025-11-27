#[cfg(test)]
mod tests;

use super::DifferentialState;

pub struct SimulationOptions {
    pub time_step: f64,
    pub max_time: f64,
    pub delta: f64,
}

pub fn simulate<State, Parameters>(
    initial_state: State,
    parameters: &Parameters,
    options: SimulationOptions,
) -> Option<State>
where
    State: DifferentialState<Parameters> + Copy + std::fmt::Debug,
{
    let mut state = initial_state;
    let mut t = 0.;

    while t < options.max_time {
        let next_state = state.step(parameters, options.time_step);
        dbg!(t, next_state);
        t += options.time_step;

        if state.distance(&next_state) < options.delta {
            return Some(next_state);
        }

        state = next_state;
    }

    None
}
