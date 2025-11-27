use approx::assert_relative_eq;

use crate::simulate::differential::{
    fixedpoint::{simulate, SimulationOptions},
    DifferentialState, StateDerivative,
};

#[test]
fn fixed_point_1d() {
    #[derive(Debug, Clone, Copy)]
    struct State(f64);
    #[derive(Debug)]
    struct Derivative(f64);
    #[derive(Debug)]
    struct Parameters(f64);

    impl DifferentialState<Parameters, Derivative> for State {
        fn derive(&self, parameters: &Parameters) -> Derivative {
            Derivative(self.0 * parameters.0)
        }

        fn step(&self, _: &Parameters, derivative: &Derivative, t: f64) -> Self {
            State(self.0 + t * derivative.0)
        }

        fn distance(&self, other: &Self) -> f64 {
            (self.0 - other.0).abs()
        }
    }

    impl StateDerivative for Derivative {
        fn abs(&self) -> f64 {
            self.0.abs()
        }
    }

    let result = simulate(
        State(1.),
        &Parameters(-1.),
        SimulationOptions {
            time_step: 1e-3,
            max_time: 1e3,
            delta: 1e-9,
        },
    )
    .expect("did not find a fixed point");

    assert_relative_eq!(result.0, 0., epsilon = 1e-9);
}
