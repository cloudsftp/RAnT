use std::thread;

pub mod adapters;
pub mod generators;

pub trait VectorGenerator {
    type Vector;

    fn generate_scan_vectors(&self) -> impl Iterator<Item = Self::Vector>;
    fn size_hint(&self) -> usize;
}

pub trait ParallelVectorGenerator {
    type Vector;

    fn generate_scan_vectors(&self) -> impl Iterator<Item = Vec<Self::Vector>>;
    fn num_chunks(&self) -> usize;
    fn size_hint(&self) -> usize;
}

pub trait ParameterAdapter<State, Parameters> {
    type Vector;

    fn compute_initial_state_and_parameters(&self, vector: Self::Vector) -> (State, Parameters);
}

pub fn scan<Vector, State, Parameters, Result>(
    vector_generator: impl VectorGenerator<Vector = Vector>,
    parameter_adapter: impl ParameterAdapter<State, Parameters, Vector = Vector>,
    simulate: impl Fn(State, &Parameters) -> Result,
) -> Vec<(State, Parameters, Result)>
where
    State: Default + Clone,
{
    let scan_points = vector_generator.generate_scan_vectors();
    let mut results = Vec::with_capacity(vector_generator.size_hint());

    for scan_point in scan_points {
        let (initial_state, parameters) =
            parameter_adapter.compute_initial_state_and_parameters(scan_point);

        let result = simulate(initial_state.clone(), &parameters);
        results.push((initial_state, parameters, result));
    }

    results
}

// TODO: remove expects
pub fn scan_parallel<Vector, State, Parameters, Result>(
    vector_generator: impl ParallelVectorGenerator<Vector = Vector> + Send + 'static,
    parameter_adapter: impl ParameterAdapter<State, Parameters, Vector = Vector>
        + Clone
        + Send
        + Sync
        + 'static,
    simulate: impl Fn(State, &Parameters) -> Result + Clone + Send + 'static,
) -> Vec<(State, Parameters, Result)>
where
    Vector: Send + 'static,
    State: Default + Clone + Send + 'static,
    Parameters: Send + 'static,
    Result: Send + 'static,
{
    let num_workers = 4; // TODO: as optional parameter, else depending on processor

    let mut results = Vec::with_capacity(vector_generator.size_hint());

    let (scan_vector_sender, scan_vector_receiver) =
        crossbeam_channel::bounded::<Vec<Vector>>(num_workers);
    let (result_sender, result_receiver) =
        crossbeam_channel::bounded::<Vec<(State, Parameters, Result)>>(num_workers);

    let mut worker_threads = Vec::with_capacity(num_workers);
    for _ in 0..num_workers {
        let scan_vector_receiver = scan_vector_receiver.clone();
        let result_sender = result_sender.clone();
        let parameter_adapter = parameter_adapter.clone();
        let simulate = simulate.clone();

        let worker_thread = thread::spawn(move || {
            for scan_vector_chunk in scan_vector_receiver {
                let mut results = Vec::with_capacity(scan_vector_chunk.len());
                for scan_vector in scan_vector_chunk {
                    let (initial_state, parameters) =
                        parameter_adapter.compute_initial_state_and_parameters(scan_vector);
                    let result = simulate(initial_state.clone(), &parameters);
                    results.push((initial_state, parameters, result));
                }
                result_sender.send(results).expect("could not send results");
            }
        });
        worker_threads.push(worker_thread);
    }

    let num_chunks = vector_generator.num_chunks();
    let scan_point_thread = thread::spawn(move || {
        let scan_point_chunks = vector_generator.generate_scan_vectors();
        for chunk in scan_point_chunks {
            scan_vector_sender
                .send(chunk)
                .expect("could not send scan vector chunk")
        }
    });

    for _ in 0..num_chunks {
        let mut result_chunk = result_receiver.recv().expect("could not receive result");
        results.append(&mut result_chunk);
    }

    scan_point_thread
        .join()
        .expect("could not join thread that sends scan points");
    for worker_thread in worker_threads {
        worker_thread.join().expect("could not join worker thread");
    }

    results
}
