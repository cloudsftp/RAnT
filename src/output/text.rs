use std::{fs::OpenOptions, io::Write};

use crate::simulate::period::Cycle;

use std::io::Result;

pub fn write_results<S, P>(
    results: Vec<(S, P, Cycle<S>)>,
    projection: impl Fn(&S, &P, &Cycle<S>) -> Option<String>,
    file_name: &str,
) -> Result<()> {
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
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
