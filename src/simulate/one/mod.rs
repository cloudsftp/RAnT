#[cfg(test)]
mod test;

pub struct SimulationOptions {
    pub iterations: usize,
    pub max_period: usize,
}

#[derive(Debug)]
pub struct SimulationResult {
    cycle: Cycle,
}

#[derive(Debug, PartialEq)]
pub enum Cycle {
    FixedPoint(f64),
    Cycle(Vec<f64>),
    Divergence,
}

impl Cycle {
    fn from_history(history: &[f64], first_encounter: usize, current_position: usize) -> Self {
        let cycle_length = if first_encounter < current_position {
            current_position - first_encounter - 1
        } else {
            current_position + history.len() - first_encounter
        };

        if cycle_length == 0 {
            return Cycle::FixedPoint(history[first_encounter]);
        }

        let mut cycle = Vec::with_capacity(cycle_length);
        if first_encounter < current_position {
            for i in first_encounter..current_position {
                cycle.push(history[i]);
            }
        } else {
            for i in first_encounter..history.len() {
                cycle.push(history[i]);
            }
            for i in 0..current_position {
                cycle.push(history[i]);
            }
        }

        Cycle::Cycle(cycle)
    }
}

pub fn simulate_function<P>(
    f: impl Fn(f64, &P) -> f64,
    initial_state: f64,
    parameters: P,
    simulation_options: &SimulationOptions,
) -> SimulationResult {
    let mut history = vec![f64::NEG_INFINITY; simulation_options.max_period];
    let mut x = initial_state;

    for i in 0..simulation_options.iterations {
        let history_index = i % simulation_options.max_period;

        let first_encounter = history.iter().position(|h| *h == x);
        if let Some(first_encounter) = first_encounter {
            return SimulationResult {
                cycle: Cycle::from_history(&history, first_encounter, history_index),
            };
        }

        history[history_index] = x;
        x = f(x, &parameters);
    }

    SimulationResult {
        cycle: Cycle::Divergence, // TODO: rotate history and return it also
    }
}
