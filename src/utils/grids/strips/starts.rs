pub struct StartIterator {
    dimensions: Vec<isize>,
    direction: Vec<isize>,
    current_plane: Vec<isize>,
    starting_position: Vec<isize>,
    pub current_position: Vec<isize>,
}

impl StartIterator {
    pub fn new(dimensions: Vec<isize>, direction: Vec<isize>) -> StartIterator {
        let dimensions_sizes = dimensions.len();
        let mut result = StartIterator {
            dimensions,
            direction,
            current_plane: vec![0; dimensions_sizes],
            starting_position: vec![0; dimensions_sizes],
            current_position: vec![0; dimensions_sizes],
        };
        if dimensions_sizes > 0 {
            result.current_plane[0] = -1;
        }
        result
    }

    pub fn empty() -> StartIterator {
        StartIterator::new(Vec::new(), Vec::new())
    }

    fn iterate_planes(&mut self) -> Option<()> {
        for index in 0..self.current_plane.len() {
            self.current_plane[index] += 1;
            if self.current_plane[index] == 1 && self.dimensions[index] <= 1 {
                continue;
            }
            if self.current_plane[index] < 2 {
                return Some(());
            }
            self.current_plane[index] = 0;
        }
        None
    }

    fn iterate_relevant_planes(&mut self) -> Option<()> {
        while let Some(()) = self.iterate_planes() {
            let plane_contains_direction = self
                .current_plane
                .iter()
                .zip(self.direction.iter())
                .all(|(&plane, &orientation)| orientation == 0 || plane != 0);
            if !plane_contains_direction {
                return Some(());
            }
        }
        None
    }

    fn set_starting_position(&mut self) {
        self.starting_position = self
            .dimensions
            .iter()
            .zip(self.direction.iter())
            .zip(self.current_plane.iter())
            .map(|((dimension, orientation), &plane)| {
                if *orientation < 0 {
                    dimension - 1 - plane
                } else {
                    plane
                }
            })
            .collect();
        self.current_position = self.starting_position.clone();
    }

    fn iterate_positions(&mut self) -> Option<()> {
        for index in 0..self.current_position.len() {
            if self.current_plane[index] <= 0 {
                continue;
            }
            if self.direction[index] < 0 {
                self.current_position[index] -= 1;
                if self.current_position[index] >= 0 {
                    return Some(());
                }
            } else {
                self.current_position[index] += 1;
                if self.current_position[index] < self.dimensions[index] {
                    return Some(());
                }
            }
            self.current_position[index] = self.starting_position[index];
        }
        None
    }

    pub fn iterate(&mut self) -> Option<()> {
        if let Some(()) = self.iterate_positions() {
            return Some(());
        }
        if let Some(()) = self.iterate_relevant_planes() {
            self.set_starting_position();
            return Some(());
        }
        None
    }
}

impl Iterator for StartIterator {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterate() {
            Some(()) => Some(self.current_position.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections;
    use std::iter::FromIterator;

    macro_rules! strip_start_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (dimensions, direction, expected_values) = $value;
                    let iterator = StartIterator::new(dimensions, direction);
                    let result: collections::HashSet<_> = iterator.collect();
                    let expected = collections::HashSet::from_iter(expected_values.iter().cloned());
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    strip_start_tests! {
        one_dimension: (vec![1], vec![1], [vec![0]]),
        two_dimensions_orthogonal: (vec![3, 3], vec![0, 1], [vec![0, 0], vec![1, 0], vec![2, 0]]),
        two_dimensions_diagonal: (vec![3, 3], vec![1, 1], [vec![0, 0], vec![1, 0], vec![2, 0], vec![0, 1], vec![0, 2]]),
        two_dimensions_reverse_diagonal: (vec![3, 3], vec![1, -1], [vec![0, 2], vec![1, 2], vec![2, 2], vec![0, 1], vec![0, 0]]),
        dimension_size_one: (vec![3, 1], vec![1, 1], [vec![0, 0], vec![1, 0], vec![2, 0]]),
    }
}
