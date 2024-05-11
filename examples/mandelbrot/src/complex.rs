#[derive(Debug, Clone, Copy, Default)]
pub struct C {
    pub a: f64,
    pub b: f64,
}

impl C {
    pub fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }

    pub fn plus_mut(&mut self, other: &Self) {
        self.a += other.a;
        self.b += other.b;
    }

    pub fn plus_move(self, other: &Self) -> Self {
        Self {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }

    pub fn sqr_mut(&mut self) {
        let a = self.a;
        self.a = a.powi(2) - self.b.powi(2);
        self.b = 2. * a * self.b;
    }

    pub fn sqr_move(self) -> Self {
        Self {
            a: self.a.powi(2) + self.b.powi(2),
            b: 2. * (self.a + self.b),
        }
    }
}
