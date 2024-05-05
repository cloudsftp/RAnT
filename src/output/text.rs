use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use crate::simulate::SimulationResult;

use std::io::Result;

pub fn write_results<S, P>(
    results: Vec<(S, P, SimulationResult<S>)>,
    projection: impl Fn(&S, &P, &SimulationResult<S>) -> Option<String>,
    file_name: &str,
) -> Result<()> {
    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_name)?;

    let lines = results
        .iter()
        .filter_map(|(s, p, res)| projection(s, p, res));
    for line in lines {
        output_file.write_all(line.as_bytes())?;
        output_file.write_all("\n".as_bytes())?;
    }

    Ok(())
}
