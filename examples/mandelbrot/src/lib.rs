use complex::C;

pub mod complex;

fn mandelbrot_mut(z: &mut C, a: &C) {
    z.sqr_mut();
    z.plus_mut(a);
}

fn mandelbrot_move(z: C, a: &C) -> C {
    let z2 = z.sqr_move();
    z2.plus_move(a)
}

const ITERATIONS: usize = 1_000;

pub fn simulate_mut(initial_state: &C, parameters: &C) -> C {
    let mut z = initial_state.clone();
    for _ in 0..ITERATIONS {
        mandelbrot_mut(&mut z, parameters)
    }
    z
}

pub fn simulate_move(initial_state: &C, parameters: &C) -> C {
    let mut z = initial_state.clone();
    for _ in 0..ITERATIONS {
        z = mandelbrot_move(z, parameters);
    }
    z
}
