use complex::C;

pub mod complex;

pub fn mandelbrot(z: C, a: &C) -> C {
    *a + z.square()
}

const ITERATIONS: usize = 1_000;

pub fn condition(z: &C) -> bool {
    false
}

pub fn simulate(initial_state: C, parameters: &C) -> Option<usize> {
    let mut x = initial_state;

    for i in 0..ITERATIONS {
        x = mandelbrot(x, &parameters);

        if condition(&x) {
            return Some(i);
        }
    }

    None
}
