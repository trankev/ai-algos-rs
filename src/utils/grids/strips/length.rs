pub fn length(dimensions: &[isize], direction: &[isize], start: &[isize]) -> isize {
    direction
        .iter()
        .zip(dimensions.iter())
        .zip(start.iter())
        .filter(|((&direction, _dimension), _start)| direction != 0)
        .map(|((&direction, &dimension), &start)| {
            if direction < 0 {
                start + 1
            } else {
                dimension as isize - start
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! length_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (dimensions, direction, start, expected) = $value;
                    let result = length(dimensions, direction, start);
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    length_tests! {
        one_dimension: (&vec![3], &vec![1], &vec![0], 3),
        two_dimensions_orthogonal: (&vec![3, 3], &vec![1, 0], &vec![0, 2], 3),
        two_dimensions_diagonal: (&vec![3, 3], &vec![1, 1], &vec![0, 1], 2),
        two_dimensions_reverse_diagonal: (&vec![3, 3], &vec![1, -1], &vec![0, 1], 2),
    }
}
