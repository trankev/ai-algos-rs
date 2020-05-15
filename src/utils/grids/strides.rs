use num;

pub fn compute_strides<T: num::PrimInt>(dimensions: &[T]) -> Vec<T> {
    dimensions
        .iter()
        .scan(num::one::<T>(), |acc, value| {
            let result = *acc;
            *acc = *acc * *value;
            Some(result)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comnpute_strides() {
        let result = compute_strides(&vec![1, 3, 5, 7]);
        assert_eq!(result, vec![1, 1, 3, 15]);
    }
}
