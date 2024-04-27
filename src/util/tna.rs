use std::{cmp::Ordering, collections::HashMap, fmt::Debug, fs::read_to_string};

use thiserror::Error;

use crate::simulate::SimulationResult;

pub fn read_tna_file(path: &str) -> anyhow::Result<Vec<Vec<f64>>> {
    let content = read_to_string(path)?;

    let lines = content.lines().filter(|line| {
        let line_content: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        !(line_content.is_empty() || line_content.starts_with('#'))
    });

    Ok(lines
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|part| part.parse::<f64>())
                .collect::<Result<Vec<f64>, _>>()
        })
        .collect::<Result<Vec<Vec<f64>>, _>>()?)
}

#[derive(Debug, Error)]
enum TnaError {
    #[error("too few values in line")]
    TooFewValues,
}

pub fn read_tna_periods_file<S, P>(
    path: &str,
    unproject_initial_state_and_parameters: impl Fn(Vec<f64>) -> anyhow::Result<(S, P)>,
) -> anyhow::Result<Vec<(S, P, usize)>> {
    read_tna_file(path)?
        .into_iter()
        .map(|mut line| -> anyhow::Result<(S, P, usize)> {
            let period = line.pop().ok_or(TnaError::TooFewValues)?;
            let (state, parameters) = unproject_initial_state_and_parameters(line)?;
            Ok((state, parameters, period as usize))
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub struct ComparisonOptions {
    pub delta: f64,
}

pub fn assert_equals_tna_periods<S, P>(
    mut rant_result: Vec<(S, P, SimulationResult<S>)>,
    mut ant_result: Vec<(S, P, usize)>,
    compare_states: impl Fn(&S, &S) -> Ordering,
    compare_parameters: impl Fn(&P, &P) -> Ordering,
    comparison_options: ComparisonOptions,
) where
    S: Debug,
    P: Debug,
{
    let sort_by_state_and_parameters = |a: (&S, &P), b: (&S, &P)| {
        let state_ordering = compare_states(a.0, b.0);
        match state_ordering {
            Ordering::Equal => compare_parameters(a.1, b.1),
            order => order,
        }
    };

    rant_result.sort_by(|a, b| sort_by_state_and_parameters((&a.0, &a.1), (&b.0, &b.1)));
    ant_result.sort_by(|a, b| sort_by_state_and_parameters((&a.0, &a.1), (&b.0, &b.1)));

    for ((_, rant_parameters, rant_result), (_, ant_parameters, ant_result)) in
        rant_result.into_iter().zip(ant_result)
    {
        match rant_result.cycle {
            crate::simulate::Cycle::FixedPoint(p) => assert_eq!(ant_result, 1),
            crate::simulate::Cycle::Cycle(c) => assert_eq!(ant_result, c.len()),
            crate::simulate::Cycle::Divergence => assert_eq!(
                ant_result, 0,
                "RAnT diverges for parameters {:?}, AnT does not (parameters: {:?})",
                rant_parameters, ant_parameters
            ),
        }
    }
}
