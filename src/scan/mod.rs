pub mod generators;
pub mod one;

pub trait VectorGenerator {
    type Vector;

    fn generate_scan_vectors(&self) -> impl Iterator<Item = Self::Vector>;
    fn size_hint(&self) -> usize;
}

pub fn scan<ScanVector, State, Parameters, Result>(
    vector_generator: impl VectorGenerator<Vector = ScanVector>,
    compute_initial_state_and_params: impl Fn(&ScanVector) -> (State, Parameters),
    simulate: impl Fn(&State, &Parameters) -> Result,
) -> Vec<(State, Parameters, Result)>
where
    State: Default + Copy,
{
    let scan_points = vector_generator.generate_scan_vectors();
    let mut results = Vec::with_capacity(vector_generator.size_hint());

    for scan_point in scan_points {
        let (initial_state, parameters) = compute_initial_state_and_params(&scan_point);

        let result = simulate(&initial_state, &parameters);
        results.push((initial_state, parameters, result));
    }

    results
}
