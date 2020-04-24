pub fn compute_strides(dimensions: &Vec<isize>) -> Vec<isize> {
    dimensions
        .iter()
        .scan(1, |mut acc, &value| {
            let result = *acc;
            *acc *= value;
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
