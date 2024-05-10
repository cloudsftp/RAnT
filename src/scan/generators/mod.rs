use super::VectorGenerator;

pub struct ScanOptions {
    pub resolutions: Vec<usize>,
}

pub struct VectorGenrator1D {
    pub resolution: usize,
}

impl VectorGenerator for VectorGenrator1D {
    type Vector = (usize, usize);

    fn generate_scan_vectors(&self) -> impl Iterator<Item = Self::Vector> {
        (0..=self.resolution).map(move |p| (p, self.resolution))
    }

    fn size_hint(&self) -> usize {
        self.resolution
    }
}
