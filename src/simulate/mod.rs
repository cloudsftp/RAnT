#[cfg(test)]
mod test;

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

pub fn find_cycle(
    f: impl Fn(f64) -> f64,
    initial_state: f64,
    max_iterations: usize,
    max_cycle_length: usize,
) -> Cycle {
    let mut history = vec![f64::NEG_INFINITY; max_cycle_length];
    let mut x = initial_state;

    for i in 0..max_iterations {
        let history_index = i % max_cycle_length;

        let first_encounter = history.iter().position(|h| *h == x);
        if let Some(first_encounter) = first_encounter {
            return Cycle::from_history(&history, first_encounter, history_index);
        }

        history[history_index] = x;
        x = f(x);
    }

    Cycle::Divergence
}
