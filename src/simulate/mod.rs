pub mod condition;
pub mod period;

pub trait Simulator<State, Parameters> {
    type Result;

    fn simulate(&self, initial_state: State, parameters: &Parameters) -> Self::Result;
}
