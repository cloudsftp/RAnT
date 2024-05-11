use complex::C;

pub mod complex;

fn mandelbrot(z: C, a: &C) -> C {
    let z2 = z.square();
    z2 + *a
}

const ITERATIONS: usize = 1_000;

pub fn simulate(initial_state: &C, parameters: &C) -> C {
    let mut z = *initial_state;
    for _ in 0..ITERATIONS {
        z = mandelbrot(z, parameters);
    }
    z
}
