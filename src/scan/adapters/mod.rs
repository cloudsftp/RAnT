use super::ParameterAdapter;

#[derive(Clone)]
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
        let x = lin_int(self.start, self.end, x, resolution);
        (self.construct_initial_state_and_parameters)(x)
    }
}

#[derive(Clone)]
pub struct ParameterAdapter2DEven<State, Parameters> {
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub construct_initial_state_and_parameters: fn(f64, f64) -> (State, Parameters), // TODO: think of better strategy
}

impl<State, Parameters> ParameterAdapter<State, Parameters>
    for ParameterAdapter2DEven<State, Parameters>
{
    type Vector = [(usize, usize); 2];

    fn compute_initial_state_and_parameters(
        &self,
        [(x, resolution_x), (y, resolution_y)]: Self::Vector,
    ) -> (State, Parameters) {
        let x = lin_int(self.start.0, self.end.0, x, resolution_x);
        let y = lin_int(self.start.1, self.end.1, y, resolution_y);
        (self.construct_initial_state_and_parameters)(x, y)
    }
}

fn lin_int(a: f64, b: f64, x: usize, resolution: usize) -> f64 {
    a + (x as f64 / resolution as f64) * (b - a)
}
