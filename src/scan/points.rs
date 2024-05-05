pub fn generate_scan_points(resolutions: &[usize]) -> Vec<Vec<usize>> {
    let mut scan_point = vec![0usize; resolutions.len()];
    let mut scan_points = Vec::with_capacity(
        resolutions
            .iter()
            .map(|res| res + 1)
            .reduce(|acc, e| acc * e)
            .expect("at least one dimensional"),
    );

    'outer: loop {
        scan_points.push(scan_point.clone());
        for (x, res) in scan_point.iter_mut().zip(resolutions).rev() {
            if *x < *res {
                *x += 1;
                continue 'outer;
            } else {
                *x = 0;
            }
        }
        break;
    }

    scan_points
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_dimension() {
        let resolutions = vec![5];
        let scan_points = generate_scan_points(&resolutions);
        assert_eq!(scan_points[0], vec![0]);
        assert_eq!(scan_points[1], vec![1]);
        assert_eq!(scan_points[5], vec![5]);
        assert_eq!(scan_points.len(), 6, "{:?}", scan_points);
    }

    #[test]
    fn test_two_dimensions() {
        let resolutions = vec![3, 2];
        let scan_points = generate_scan_points(&resolutions);
        assert_eq!(scan_points[0], vec![0, 0]);
        assert_eq!(scan_points[2], vec![0, 2]);
        assert_eq!(scan_points[3], vec![1, 0]);
        assert_eq!(scan_points[6], vec![2, 0]);
        assert_eq!(scan_points[7], vec![2, 1]);
        assert_eq!(scan_points[11], vec![3, 2]);
        assert_eq!(scan_points.len(), 12, "{:?}", scan_points);
    }

    #[test]
    fn test_three_dimensions() {
        let resolutions = vec![3, 2, 1];
        let scan_points = generate_scan_points(&resolutions);
        assert_eq!(scan_points[0], vec![0, 0, 0]);
        assert_eq!(scan_points[1], vec![0, 0, 1]);
        assert_eq!(scan_points[2], vec![0, 1, 0]);
        assert_eq!(scan_points[6], vec![1, 0, 0]);
        assert_eq!(scan_points[7], vec![1, 0, 1]);
        assert_eq!(scan_points.len(), 24, "{:?}", scan_points);
    }
}
