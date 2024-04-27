#[cfg(test)]
mod test;

pub struct SimulationOptions {
    pub iterations: usize,
    pub max_period: usize,
    pub delta: f64,
}

#[derive(Debug)]
pub struct SimulationResult<S> {
    pub cycle: Cycle<S>,
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

pub fn simulate_function<S, P>(
    f: impl Fn(S, &P) -> S,
    d: impl Fn(&S, &S) -> f64,
    initial_state: S,
    parameters: &P,
    simulation_options: &SimulationOptions,
) -> SimulationResult<S>
where
    S: Default + Copy,
{
    let history_length = simulation_options.max_period;
    let mut history = vec![S::default(); history_length];
    let mut x = initial_state;

    for _ in 0..simulation_options.iterations - history_length {
        x = f(x, parameters);
    }

    for i in 0..history_length {
        // TODO: earlier checks for cycles
        history[i] = x;
        x = f(x, parameters);
    }

    let last_encounter = history
        .iter()
        .rev()
        .take(simulation_options.max_period)
        .position(|h| d(h, &x) < simulation_options.delta)
        .map(|pos| history_length - pos - 1);

    match last_encounter {
        Some(last_encounter) => SimulationResult {
            cycle: Cycle::from_history(&history, last_encounter),
        },
        None => SimulationResult {
            cycle: Cycle::Divergence, // TODO: rotate history and return it also
        },
    }
}
