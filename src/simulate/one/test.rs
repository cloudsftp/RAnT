use super::*;

struct CycleFromHistoryTestCase<'a> {
    history: &'a [f64],
    first_encounter: usize,
    current_position: usize,
    expected: Cycle,
}

#[test]
fn cycle_from_history() {
    let test_cases = [
        CycleFromHistoryTestCase {
            history: &[0., f64::NEG_INFINITY],
            first_encounter: 0,
            current_position: 1,
            expected: Cycle::FixedPoint(0.),
        },
        CycleFromHistoryTestCase {
            history: &[0., 1., f64::NEG_INFINITY],
            first_encounter: 0,
            current_position: 2,
            expected: Cycle::Cycle(vec![0., 1.]),
        },
        CycleFromHistoryTestCase {
            history: &[0., 1., 2., f64::NEG_INFINITY],
            first_encounter: 0,
            current_position: 3,
            expected: Cycle::Cycle(vec![0., 1., 2.]),
        },
        CycleFromHistoryTestCase {
            history: &[-1., 0., 1., 2., f64::NEG_INFINITY],
            first_encounter: 1,
            current_position: 4,
            expected: Cycle::Cycle(vec![0., 1., 2.]),
        },
        CycleFromHistoryTestCase {
            history: &[-2., -1., 0., 1., 2., 3.],
            first_encounter: 2,
            current_position: 0,
            expected: Cycle::Cycle(vec![0., 1., 2., 3.]),
        },
        CycleFromHistoryTestCase {
            history: &[-2., -1., 0., 1., 2., 3.],
            first_encounter: 2,
            current_position: 1,
            expected: Cycle::Cycle(vec![0., 1., 2., 3., -2.]),
        },
        CycleFromHistoryTestCase {
            history: &[-2., -1., 0., 1., 2., 3.],
            first_encounter: 2,
            current_position: 2,
            expected: Cycle::Cycle(vec![0., 1., 2., 3., -2., -1.]),
        },
    ];

    for test_case in test_cases {
        let cycle = Cycle::from_history(
            test_case.history,
            test_case.first_encounter,
            test_case.current_position,
        );
        assert_eq!(cycle, test_case.expected);
    }
}
