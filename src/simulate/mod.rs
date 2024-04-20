#[derive(Debug)]
pub enum Cycle {
    FixedPoint(f64),
    Cycle(Vec<f64>),
    Divergence,
}

const MAX_ITERATIONS: u64 = 1_000;

pub fn find_cycle(f: impl Fn(f64) -> f64, initial_state: f64) -> Cycle {
    let mut x = initial_state;
    for i in 0..MAX_ITERATIONS {
        let last = x;
        x = f(x);

        if last == x {
            return Cycle::FixedPoint(last);
        }
    }

    Cycle::Divergence
}
