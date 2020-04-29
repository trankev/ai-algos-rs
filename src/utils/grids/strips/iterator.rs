use crate::utils::grids;
use crate::utils::grids::strips;

pub struct StripIterator {
    dimensions: Vec<isize>,
    directions: grids::DirectionIterator,
    strip_starts: strips::StartIterator,
    pub strips: strips::Indices,
}

impl StripIterator {
    pub fn new(dimensions: Vec<usize>) -> StripIterator {
        let directions = grids::DirectionIterator::new(dimensions.len());
        StripIterator {
            dimensions: dimensions.iter().map(|&x| x as isize).collect(),
            directions,
            strip_starts: strips::StartIterator::empty(),
            strips: strips::Indices::empty(),
        }
    }

    pub fn iterate(&mut self) -> Option<()> {
        if let Some(()) = self.strip_starts.iterate() {
            self.strips = strips::Indices::new(
                &self.dimensions,
                &self.directions.current_value,
                &self.strip_starts.current_position,
            );
            return Some(());
        }
        if let Some(()) = self.directions.iterate_forward() {
            self.strip_starts = strips::StartIterator::new(
                self.dimensions.clone(),
                self.directions.current_value.clone(),
            );
            return self.iterate();
        }
        None
    }
}

impl Iterator for StripIterator {
    type Item = strips::Indices;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterate() {
            Some(()) => Some(self.strips.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections;

    #[test]
    fn test_iteration() {
        let iterator = StripIterator::new(vec![3, 3]);
        let strips = iterator
            .map(|strip| strip.collect::<Vec<_>>())
            .collect::<collections::HashSet<_>>();
        let expected = [
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![0, 3, 6],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![6],
            vec![3, 7],
            vec![0, 4, 8],
            vec![1, 5],
            vec![2],
            vec![0],
            vec![1, 3],
            vec![2, 4, 6],
            vec![5, 7],
            vec![8],
        ]
        .iter()
        .cloned()
        .collect::<collections::HashSet<_>>();
        assert_eq!(strips, expected);
    }
}
