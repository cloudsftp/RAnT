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
    fn from_history(history: &[S], first_encounter: usize, current_position: usize) -> Self {
        let cycle_length = if first_encounter < current_position {
            current_position - first_encounter - 1
        } else {
            current_position + history.len() - first_encounter
        };

        if cycle_length == 0 {
            return Cycle::FixedPoint(history[first_encounter]);
        }

        // TODO: test performance against implementation with for loops and pushing to vec
        let cycle = if first_encounter < current_position {
            history
                .iter()
                .take(current_position)
                .skip(first_encounter)
                .copied()
                .collect()
        } else {
            history
                .iter()
                .skip(first_encounter)
                .chain(history.iter().take(current_position))
                .copied()
                .collect()
        };

        Cycle::Cycle(cycle)
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
    let mut history = vec![S::default(); simulation_options.max_period];
    let mut x = initial_state;

    for i in 0..simulation_options.iterations {
        let history_index = i % simulation_options.max_period;

        let first_encounter = history
            .iter()
            .position(|h| d(h, &x) < simulation_options.delta);
        if let Some(first_encounter) = first_encounter {
            return SimulationResult {
                cycle: Cycle::from_history(&history, first_encounter, history_index),
            };
        }

        history[history_index] = x;
        x = f(x, parameters);
    }

    SimulationResult {
        cycle: Cycle::Divergence, // TODO: rotate history and return it also
    }
}
