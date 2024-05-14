use complex::C;
use rant::simulate::condition::{simulate, SimulationOptions};

pub mod complex;

pub fn mandelbrot(z: C, a: &C) -> C {
    *a + z.square()
}

pub fn condition(_z: &C) -> bool {
    false
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
