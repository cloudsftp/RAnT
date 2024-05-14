pub struct SimulationOptions {
    pub max_iterations: usize,
}

pub fn simulate<State, Parameters>(
    initial_state: State,
    parameters: &Parameters,
    function: impl Fn(State, &Parameters) -> State,
    condition: impl Fn(&State) -> bool,
    options: SimulationOptions,
) -> Option<usize> {
    let mut x = initial_state;

    for i in 0..options.max_iterations {
        x = function(x, parameters);

        if condition(&x) {
            return Some(i);
        }
    }

    None
}
