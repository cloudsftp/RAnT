use crate::{complex::C, mandelbrot};

struct FunctionTestCase {
    input: C,
    parameter: C,
    output: C,
}

#[test]
fn test_mandelbrot_function() {
    let tests = vec![
        FunctionTestCase {
            input: C::new(0., 0.),
            parameter: C::new(0., 0.),
            output: C::new(0., 0.),
        },
        FunctionTestCase {
            input: C::new(1., 0.),
            parameter: C::new(0., 0.),
            output: C::new(1., 0.),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let output = mandelbrot(test.input, &test.parameter);
        assert_eq!(&output, &test.output, "test case {}", i)
    }
}
