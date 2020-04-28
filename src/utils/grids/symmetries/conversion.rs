pub fn convert_position(
    dimensions: &[isize],
    position: &[isize],
    destination: &[bool],
    permutation: &[isize],
) -> Vec<isize> {
    permutation
        .iter()
        .enumerate()
        .map(|(index, &swap)| {
            if destination[index] {
                position[swap as usize]
            } else {
                dimensions[index] - position[swap as usize] - 1
            }
        })
        .collect()
}

#[cfg(tests)]
mod tests {
    use super::*;

    macro_rules! convert_position_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (dimensions, position, destination, permutation, expected) = $value;
                    let result = convert_position(dimensions, position, destination, permutation);
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    convert_position_tests! {
        origin_identity: ([3, 3], [0, 0], [false, false], [0, 1], [0, 0]),
        rotation: ([3, 3], [1, 2], [false, true], [1, 0], [0, 1]),
        reflection: ([3, 3], [0, 2], [false, false], [1, 0], [2, 0]),
    }
}
