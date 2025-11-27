use approx::assert_relative_eq;

use crate::simulate::differential::{
    fixedpoint::{simulate, SimulationOptions},
    DifferentialState,
};

#[test]
fn fixed_point_1d() {
    #[derive(Debug, Clone, Copy)]
    struct State(f64);
    struct Parameters(f64);
    impl DifferentialState<Parameters> for State {
        fn step(&self, parameters: &Parameters, t: f64) -> Self {
            let differential = parameters.0 * self.0;
            State(self.0 + t * differential)
        }

        fn distance(&self, other: &Self) -> f64 {
            (self.0 - other.0).abs()
        }
    }

    let result = simulate(
        State(1.),
        &Parameters(-1.),
        SimulationOptions {
            time_step: 1e-3,
            max_time: 1e9,
            delta: 1e-12,
        },
    )
    .expect("did not find a fixed point");

    assert_relative_eq!(result.0, 0., epsilon = 1e-9);
}
