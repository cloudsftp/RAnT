use super::*;

struct CycleFromHistoryTestCase<'a, S> {
    history: &'a [f64],
    last_encounter: usize,
    expected: Cycle<S>,
}

#[test]
fn cycle_from_history() {
    let test_cases = [
        CycleFromHistoryTestCase {
            history: &[0.],
            last_encounter: 0,
            expected: Cycle::FixedPoint(0.),
        },
        CycleFromHistoryTestCase {
            history: &[0., 1.],
            last_encounter: 0,
            expected: Cycle::Cycle(vec![0., 1.]),
        },
        CycleFromHistoryTestCase {
            history: &[0., 1., 2.],
            last_encounter: 1,
            expected: Cycle::Cycle(vec![1., 2.]),
        },
    ];

    for test_case in test_cases {
        let cycle = Cycle::from_history(test_case.history, test_case.last_encounter);
        assert_eq!(cycle, test_case.expected);
    }
}
