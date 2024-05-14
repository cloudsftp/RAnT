#[cfg(test)]
mod test;

pub struct SimulationOptions {
    pub iterations: usize,
    pub max_period: usize,
    pub delta: f64,
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

pub fn simulate<State, Parameters>(
    initial_state: State,
    parameters: &Parameters,
    function: impl Fn(State, &Parameters) -> State,
    distance: impl Fn(&State, &State) -> f64,
    options: SimulationOptions,
) -> Cycle<State>
where
    State: Default + Copy,
{
    let history_length = options.max_period;
    let mut history = vec![State::default(); history_length];
    let mut x = initial_state;

    for _ in 0..options.iterations - history_length {
        x = function(x, parameters);
    }

    for item in history.iter_mut() {
        // TODO: earlier checks for cycles
        *item = x;
        x = function(x, parameters);
    }

    let last_encounter = history
        .iter()
        .rev()
        .take(options.max_period)
        .position(|h| distance(h, &x) < options.delta)
        .map(|pos| history_length - pos - 1);

    match last_encounter {
        Some(last_encounter) => Cycle::from_history(&history, last_encounter),
        None => Cycle::Divergence, // TODO: rotate history and return it also
    }
}
