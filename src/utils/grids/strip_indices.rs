use crate::utils::grids;
use crate::utils::vectors;
use std::iter;
use std::ops;

#[derive(Debug)]
pub struct StripIndices {
    step: isize,
    current: isize,
    remaining_count: isize,
}

impl StripIndices {
    pub fn new(dimensions: &[isize], direction: &[isize], origin: &[isize]) -> StripIndices {
        let strides = grids::compute_strides(dimensions);
        let step = vectors::dot_product(&strides, direction);
        let start = vectors::dot_product(&strides, origin);
        let strip_length = grids::strip_length(dimensions, direction, origin);
        StripIndices {
            current: start,
            step,
            remaining_count: strip_length,
        }
    }
}

impl Iterator for StripIndices {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{:?}", self);
        if self.remaining_count == 0 {
            return None;
        }
        let result = self.current;
        self.current += self.step;
        self.remaining_count -= 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_indices() {
        let iterator = StripIndices::new(&vec![3, 3], &vec![1, -1], &vec![0, 1]);
        let result = iterator.collect::<Vec<_>>();
        let expected = vec![3, 1];
        assert_eq!(result, expected);
    }
}
