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

const MAX_ITERATIONS: usize = 1_000;
const MAX_CYCLE_LENGTH: usize = 10;

pub fn find_cycle(f: impl Fn(f64) -> f64, initial_state: f64) -> Cycle {
    let mut history: [f64; MAX_CYCLE_LENGTH] = [f64::NEG_INFINITY; MAX_CYCLE_LENGTH];
    let mut x = initial_state;

    for i in 0..MAX_ITERATIONS {
        let history_index = i % MAX_CYCLE_LENGTH;

        let first_encounter = history.iter().position(|h| *h == x);
        if let Some(first_encounter) = first_encounter {
            return Cycle::from_history(&history, first_encounter, history_index);
        }

        history[history_index] = x;
        x = f(x);
    }

    Cycle::Divergence
}

#[cfg(test)]
mod test {
    use super::*;

    struct CycleFromHistoryTestCase<'a> {
        history: &'a [f64],
        first_encounter: usize,
        current_position: usize,
        expected: Cycle,
    }

    #[test]
    fn cycle_from_history() {
        let test_cases = [
            CycleFromHistoryTestCase {
                history: &[0., f64::NEG_INFINITY],
                first_encounter: 0,
                current_position: 1,
                expected: Cycle::FixedPoint(0.),
            },
            CycleFromHistoryTestCase {
                history: &[0., 1., f64::NEG_INFINITY],
                first_encounter: 0,
                current_position: 2,
                expected: Cycle::Cycle(vec![0., 1.]),
            },
            CycleFromHistoryTestCase {
                history: &[0., 1., 2., f64::NEG_INFINITY],
                first_encounter: 0,
                current_position: 3,
                expected: Cycle::Cycle(vec![0., 1., 2.]),
            },
            CycleFromHistoryTestCase {
                history: &[-1., 0., 1., 2., f64::NEG_INFINITY],
                first_encounter: 1,
                current_position: 4,
                expected: Cycle::Cycle(vec![0., 1., 2.]),
            },
            CycleFromHistoryTestCase {
                history: &[-2., -1., 0., 1., 2., 3.],
                first_encounter: 2,
                current_position: 0,
                expected: Cycle::Cycle(vec![0., 1., 2., 3.]),
            },
            CycleFromHistoryTestCase {
                history: &[-2., -1., 0., 1., 2., 3.],
                first_encounter: 2,
                current_position: 1,
                expected: Cycle::Cycle(vec![0., 1., 2., 3., -2.]),
            },
            CycleFromHistoryTestCase {
                history: &[-2., -1., 0., 1., 2., 3.],
                first_encounter: 2,
                current_position: 2,
                expected: Cycle::Cycle(vec![0., 1., 2., 3., -2., -1.]),
            },
        ];

        for test_case in test_cases {
            let cycle = Cycle::from_history(
                test_case.history,
                test_case.first_encounter,
                test_case.current_position,
            );
            assert_eq!(cycle, test_case.expected);
        }
    }
}
