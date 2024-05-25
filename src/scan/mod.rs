use rayon::prelude::*;

pub mod adapters;
pub mod generators;

pub trait VectorGenerator {
    type Vector;

    fn generate_scan_vectors(self) -> impl Iterator<Item = Self::Vector>;
    fn size_hint(&self) -> usize;
}

pub trait ParallelVectorGenerator {
    type Vector;

    fn generate_scan_vectors(
        self,
    ) -> impl Iterator<Item = impl Iterator<Item = Self::Vector> + Send> + Send;
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
) -> impl Iterator<Item = (State, Parameters, Result)>
where
    State: Default + Clone,
{
    let scan_points = vector_generator.generate_scan_vectors();

    scan_points.map(move |scan_point| {
        let (initial_state, parameters) =
            parameter_adapter.compute_initial_state_and_parameters(scan_point);

        let result = simulate(initial_state.clone(), &parameters);
        (initial_state, parameters, result)
    })
}

pub fn scan_parallel<Vector, State, Parameters, Result>(
    vector_generator: impl ParallelVectorGenerator<Vector = Vector> + 'static,
    parameter_adapter: impl ParameterAdapter<State, Parameters, Vector = Vector>
        + Sync
        + Send
        + Copy
        + 'static,
    simulate: impl Fn(State, &Parameters) -> Result + Sync + Send + Copy + 'static,
) -> impl ParallelIterator<Item = (State, Parameters, Result)>
where
    Vector: Send,
    State: Default + Clone + Send + Sync,
    Parameters: Send + Sync,
    Result: Send + Sync,
{
    vector_generator
        .generate_scan_vectors()
        .par_bridge()
        .map(move |scan_points| {
            scan_points.map(move |scan_point| {
                let (initial_state, parameters) =
                    parameter_adapter.compute_initial_state_and_parameters(scan_point);

                let result = simulate(initial_state.clone(), &parameters);
                (initial_state, parameters, result)
            })
        })
        .flatten_iter()
}

// TODO: remove expects
/*
pub fn scan_parallel_channels<Vector, State, Parameters, Result>(
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
    let num_workers = 12; // TODO: as optional parameter, else depending on processor

    let mut results = Vec::with_capacity(vector_generator.size_hint());

    let (scan_vector_sender, scan_vector_receiver) =
        crossbeam_channel::bounded::<Box<dyn Iterator<Item = Vector> + Send>>(num_workers);
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
                let results = scan_vector_chunk
                    .map(|scan_vector| {
                        let (initial_state, parameters) =
                            parameter_adapter.compute_initial_state_and_parameters(scan_vector);
                        let result = simulate(initial_state.clone(), &parameters);
                        (initial_state, parameters, result)
                    })
                    .collect();
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
                .send(Box::new(chunk))
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
*/
