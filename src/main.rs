use crate::simulate::find_cycle;

pub mod simulate;

fn main() {
    let a = 2.3;
    let f = |x: f64| a * x * (1f64 - x);

    println!("{:?}", find_cycle(f, 0.5))
}
