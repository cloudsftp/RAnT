use complex::C;
use rantlib::simulate::condition::{simulate, SimulationOptions};

pub mod complex;

#[cfg(test)]
mod test;

pub fn mandelbrot(z: C, a: &C) -> C {
    *a + z.square()
}

pub fn condition(z: &C) -> bool {
    z.abs2() > 1_000_000_000_000.
}

pub fn simulate_mandelbrot(initial_state: C, parameters: &C) -> Option<usize> {
    simulate(
        initial_state,
        parameters,
        mandelbrot,
        condition,
        SimulationOptions {
            max_iterations: 1_000,
        },
    )
}
