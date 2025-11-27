pub mod fixedpoint;

pub trait StateDerivative {
    fn abs(&self) -> f64;
}

pub trait DifferentialState<Parameters, Derivative> {
    fn derive(&self, parameters: &Parameters) -> Derivative;
    fn step(&self, parameters: &Parameters, derivative: &Derivative, t: f64) -> Self;
    fn distance(&self, other: &Self) -> f64;
}
