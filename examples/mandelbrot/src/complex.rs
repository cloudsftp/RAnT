use std::ops::Add;

#[derive(Debug, Clone, Copy, Default)]
pub struct C {
    pub a: f64,
    pub b: f64,
}

impl C {
    pub fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }

    pub fn square(self) -> Self {
        Self {
            a: self.a.powi(2) + self.b.powi(2),
            b: 2. * (self.a + self.b),
        }
    }
}

impl Add for C {
    type Output = C;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}
