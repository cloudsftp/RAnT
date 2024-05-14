use super::Simulator;

pub struct ConditionSimulator<State, Parameters> {
    pub max_iterations: usize,
    pub function: fn(State, &Parameters) -> State,
    pub condition: fn(&State) -> bool,
}

impl<State, Parameters> Simulator<State, Parameters> for ConditionSimulator<State, Parameters> {
    type Result = Option<usize>;

    fn simulate(&self, initial_state: State, parameters: &Parameters) -> Self::Result {
        let mut x = initial_state;

        for i in 0..self.max_iterations {
            x = (self.function)(x, &parameters);

            if (self.condition)(&x) {
                return Some(i);
            }
        }

        None
    }
}
