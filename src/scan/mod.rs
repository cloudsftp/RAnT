pub mod adapters;
pub mod generators;

pub trait VectorGenerator {
    type Vector;

    fn generate_scan_vectors(&self) -> impl Iterator<Item = Self::Vector>;
    fn size_hint(&self) -> usize;
}

pub trait ParameterAdapter<State, Parameters> {
    type Vector;

    fn compute_initial_state_and_parameters(&self, vector: Self::Vector) -> (State, Parameters);
}

pub trait Simulator<State, Parameters> {
    type Result;

    fn simulate(&self, initial_state: State, parameters: &Parameters) -> Self::Result;
}

pub fn scan<Vector, State, Parameters, Result>(
    vector_generator: impl VectorGenerator<Vector = Vector>,
    parameter_adapter: impl ParameterAdapter<State, Parameters, Vector = Vector>,
    simulator: impl Simulator<State, Parameters, Result = Result>,
) -> Vec<(State, Parameters, Result)>
where
    State: Default + Clone,
{
    let scan_points = vector_generator.generate_scan_vectors();
    let mut results = Vec::with_capacity(vector_generator.size_hint());

    for scan_point in scan_points {
        let (initial_state, parameters) =
            parameter_adapter.compute_initial_state_and_parameters(scan_point);

        let result = simulator.simulate(initial_state.clone(), &parameters);
        results.push((initial_state, parameters, result));
    }

    results
}
