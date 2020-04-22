pub fn dot_product(vector_a: &[isize], vector_b: &[isize]) -> isize {
    vector_a.iter()
        .zip(vector_b.iter())
        .map(|(value_a, value_b)| value_a * value_b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! dot_product_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (vector_a, vector_b, expected) = $value;
                    let result = dot_product(&vector_a, &vector_b);
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    dot_product_tests! {
        positive: (&[2, 3][..], &[5, 7][..], 31),
        negative: ([2, -3], [5, 7], -11),
        zero: ([0, 0], [2, 3], 0),
    }
}
