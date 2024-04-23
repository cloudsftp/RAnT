use rant::{
    scan::one::{scan_function, ScanOptions},
    simulate::SimulationOptions,
};

#[derive(Debug)]
struct Parameters {
    a: f64,
}

fn logistic(x: f64, params: &Parameters) -> f64 {
    params.a * x * (1. - x)
}

fn distance(a: &f64, b: &f64) -> f64 {
    (a - b).abs()
}

const START: f64 = 3.;
const STOP: f64 = 4.;
const RESOLUTION: f64 = 100.;
fn gen_parameters(x: usize, len: usize) -> Parameters {
    Parameters {
        a: START + (x as f64 / len as f64) * (STOP - START) / RESOLUTION,
    }
}

fn main() {
    let scan_options = ScanOptions {
        num_points: RESOLUTION as usize,
        initial_state: 0.5,
    };

    let simulation_options = SimulationOptions {
        iterations: 20_000,
        max_period: 128,
        delta: 1e-9,
    };

    let results = scan_function(
        logistic,
        distance,
        gen_parameters,
        scan_options,
        simulation_options,
    );

    println!("{:?}", results)
}

#[cfg(test)]
mod test {
    use anyhow::anyhow;
    use rant::util::tna::read_tna_periods_file;

    use super::*;

    fn unproject_initial_state_and_parameters(
        values: Vec<f64>,
    ) -> anyhow::Result<(f64, Parameters)> {
        if values.len() == 1 {
            Ok((0.5, Parameters { a: values[0] }))
        } else {
            Err(anyhow!("expected exactly 1 value"))
        }
    }

    #[test]
    fn period_test() {
        let max_period = 128;
        let iterations = 20_000;

        let periods = read_tna_periods_file(
            "ant/test_data/period.tna",
            unproject_initial_state_and_parameters,
        );
        dbg!(periods);
        assert!(false)
    }
}
