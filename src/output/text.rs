use std::{fs::OpenOptions, io::Write};

use std::io::Result;

pub fn write_results<State, Parameters, SimulationResult>(
    results: impl Iterator<Item = (State, Parameters, SimulationResult)>,
    projection: impl Fn(State, Parameters, SimulationResult) -> Option<String>,
    file_name: &str,
) -> Result<()> {
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file_name)?;

    let lines = results.filter_map(|(s, p, res)| projection(s, p, res));
    for line in lines {
        output_file.write_all(line.as_bytes())?;
        output_file.write_all("\n".as_bytes())?;
    }

    Ok(())
}
