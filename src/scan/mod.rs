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

pub fn scan<Vector, State, Parameters, Result>(
    vector_generator: impl VectorGenerator<Vector = Vector>,
    parameter_adapter: impl ParameterAdapter<State, Parameters, Vector = Vector>,
    simulate: impl Fn(&State, &Parameters) -> Result,
) -> Vec<(State, Parameters, Result)>
where
    State: Default + Copy,
{
    let scan_points = vector_generator.generate_scan_vectors();
    let mut results = Vec::with_capacity(vector_generator.size_hint());

    for scan_point in scan_points {
        let (initial_state, parameters) =
            parameter_adapter.compute_initial_state_and_parameters(scan_point);

        let result = simulate(&initial_state, &parameters);
        results.push((initial_state, parameters, result));
    }

    results
}
