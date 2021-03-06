use crate::utils::grids;
use crate::utils::grids::strips;
use crate::utils::vectors;

#[derive(Clone, Debug)]
pub struct Indices {
    pub start: isize,
    pub length: isize,
    pub step: isize,
    current: isize,
    remaining_count: isize,
}

impl Indices {
    pub fn new(dimensions: &[isize], direction: &[isize], origin: &[isize]) -> Indices {
        let strides = grids::compute_strides(dimensions);
        let step = vectors::dot_product(&strides, direction);
        let start = vectors::dot_product(&strides, origin);
        let length = strips::length(dimensions, direction, origin);
        Indices {
            current: start,
            start,
            length,
            step,
            remaining_count: length,
        }
    }

    pub fn empty() -> Indices {
        Indices {
            step: 0,
            current: 0,
            start: 0,
            length: 0,
            remaining_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.current = self.start;
        self.remaining_count = self.length;
    }
}

impl Iterator for Indices {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_count == 0 {
            return None;
        }
        let result = self.current as usize;
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
        let iterator = Indices::new(&vec![3, 3], &vec![1, -1], &vec![0, 1]);
        let result = iterator.collect::<Vec<_>>();
        let expected = vec![3, 1];
        assert_eq!(result, expected);
    }
}
