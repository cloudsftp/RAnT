pub mod fixedpoint;

pub trait DifferentialState<Parameters> {
    fn step(&self, parameters: &Parameters, t: f64) -> Self;
    fn distance(&self, other: &Self) -> f64;
}
