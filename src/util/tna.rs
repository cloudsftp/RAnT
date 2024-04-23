use std::fs::read_to_string;

use thiserror::Error;

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
