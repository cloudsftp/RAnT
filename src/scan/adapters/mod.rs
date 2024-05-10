use super::ParameterAdapter;

pub struct ParameterAdapter1DEven<State, Parameters> {
    pub start: f64,
    pub end: f64,
    pub construct_initial_state_and_parameters: fn(f64) -> (State, Parameters), // TODO: think of better strategy
}

impl<State, Parameters> ParameterAdapter<State, Parameters>
    for ParameterAdapter1DEven<State, Parameters>
{
    type Vector = (usize, usize);

    fn compute_initial_state_and_parameters(
        &self,
        (x, resolution): Self::Vector,
    ) -> (State, Parameters) {
        let x = self.start + (x as f64 / resolution as f64) * (self.end - self.start);
        (self.construct_initial_state_and_parameters)(x)
    }
}
