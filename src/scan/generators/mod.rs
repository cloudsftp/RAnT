use super::VectorGenerator;

pub struct ScanOptions {
    pub resolutions: Vec<usize>,
}

pub struct VectorGenerator1D {
    pub resolution: usize,
}

impl VectorGenerator for VectorGenerator1D {
    type Vector = (usize, usize);

    fn generate_scan_vectors(&self) -> impl Iterator<Item = Self::Vector> {
        (0..=self.resolution).map(move |p| (p, self.resolution))
    }

    fn size_hint(&self) -> usize {
        self.resolution
    }
}

pub struct VectorGenerator2D {
    pub resolution: (usize, usize),
}

impl VectorGenerator for VectorGenerator2D {
    type Vector = [(usize, usize); 2];

    fn generate_scan_vectors(&self) -> impl Iterator<Item = Self::Vector> {
        let (resolution_x, resolution_y) = self.resolution;
        (0..=resolution_x)
            .map(move |x| (0..=resolution_y).map(move |y| [(x, resolution_x), (y, resolution_y)]))
            .flatten()
    }

    fn size_hint(&self) -> usize {
        self.resolution.0 * self.resolution.1
    }
}
