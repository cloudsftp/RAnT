use rant::{
    scan::one::{scan_1function, ScanOptions},
    simulate::one::SimulationOptions,
};

struct Parameters {
    a: f64,
}

fn logistic(x: f64, params: &Parameters) -> f64 {
    params.a * x * (1. - x)
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
    };

    let results = scan_1function(logistic, gen_parameters, scan_options, simulation_options);

    println!("{:?}", results)
}

#[cfg(test)]
mod test {
    #[test]
    fn period_test() {
        let max_period = 128;
        let iterations = 20_000;
    }
}
