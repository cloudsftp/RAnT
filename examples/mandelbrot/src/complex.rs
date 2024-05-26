use std::ops::Add;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
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
            a: self.a.powi(2) - self.b.powi(2),
            b: 2. * self.a * self.b,
        }
    }

    pub fn abs2(self) -> f64 {
        self.a.powi(2) + self.b.powi(2)
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

#[cfg(test)]
mod test {
    use super::*;

    struct SquareTestCase {
        input: C,
        output: C,
    }

    #[test]
    fn test_square() {
        let tests = vec![
            SquareTestCase {
                input: C::new(0., 0.),
                output: C::new(0., 0.),
            },
            SquareTestCase {
                input: C::new(1., 0.),
                output: C::new(1., 0.),
            },
            SquareTestCase {
                input: C::new(0., 1.),
                output: C::new(-1., 0.),
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            let output = test.input.square();
            assert_eq!(output, test.output, "test case {}", i);
        }
    }
}
