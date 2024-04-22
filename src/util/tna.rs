use std::fs::read_to_string;

pub fn read_tna_file(path: &str) -> anyhow::Result<Vec<Vec<f64>>> {
    let content = read_to_string(path)?;

    let lines = content.lines().filter(|line| {
        let line_content: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        !(line_content.is_empty() || line_content.starts_with("#"))
    });

    Ok(lines
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|part| part.parse::<f64>())
                .collect::<Result<Vec<f64>, _>>()
        })
        .collect::<Result<Vec<Vec<f64>>, _>>()?)
}
