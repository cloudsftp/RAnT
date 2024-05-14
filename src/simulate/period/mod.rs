use super::Simulator;

#[cfg(test)]
mod test;

pub struct SimulationOptions {
    pub iterations: usize,
    pub max_period: usize,
    pub delta: f64,
}

pub struct PeriodSimulator<State, Parameters> {
    function: fn(State, &Parameters) -> State,
    distance: fn(&State, &State) -> f64,
    simulation_options: SimulationOptions,
}

#[derive(Debug, PartialEq)]
pub enum Cycle<S> {
    FixedPoint(S),
    Cycle(Vec<S>),
    Divergence,
}

impl<S> Cycle<S>
where
    S: Copy,
{
    fn from_history(history: &[S], last_encounter: usize) -> Self {
        let cycle_length = history.len() - last_encounter - 1;

        match cycle_length {
            0 => Cycle::FixedPoint(history[last_encounter]),
            _ => Cycle::Cycle(history.iter().skip(last_encounter).copied().collect()),
        }
    }
}

impl<State, Parameters> Simulator<State, Parameters> for PeriodSimulator<State, Parameters>
where
    State: Default + Copy,
{
    type Result = Cycle<State>;

    fn simulate(&self, initial_state: State, parameters: &Parameters) -> Self::Result {
        let history_length = self.simulation_options.max_period;
        let mut history = vec![State::default(); history_length];
        let mut x = initial_state;

        for _ in 0..self.simulation_options.iterations - history_length {
            x = (self.function)(x, parameters);
        }

        for item in history.iter_mut() {
            // TODO: earlier checks for cycles
            *item = x;
            x = (self.function)(x, parameters);
        }

        let last_encounter = history
            .iter()
            .rev()
            .take(self.simulation_options.max_period)
            .position(|h| (self.distance)(h, &x) < self.simulation_options.delta)
            .map(|pos| history_length - pos - 1);

        match last_encounter {
            Some(last_encounter) => Cycle::from_history(&history, last_encounter),
            None => Cycle::Divergence, // TODO: rotate history and return it also
        }
    }
}
