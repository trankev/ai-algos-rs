use crate::utils::vectors;

pub struct DirectionIterator {
    pub current_value: Vec<isize>,
    positive_plane: Vec<isize>,
}

impl DirectionIterator {
    pub fn new(dimensions: usize) -> DirectionIterator {
        let mut result = DirectionIterator {
            current_value: vec![-1; dimensions],
            positive_plane: (0..dimensions)
                .map(|exponent| 2isize.pow(exponent as u32))
                .collect(),
        };
        result.current_value[0] = -2;
        result
    }

    pub fn iterate(&mut self) -> Option<()> {
        for index in 0..self.current_value.len() {
            self.current_value[index] += 1;
            if self.current_value[index] <= 1 {
                return Some(());
            }
            self.current_value[index] = -1;
        }
        None
    }

    pub fn iterate_forward(&mut self) -> Option<()> {
        while let Some(()) = self.iterate() {
            let cross = vectors::dot_product(&self.current_value, &self.positive_plane);
            if cross > 0 {
                return Some(());
            }
        }
        None
    }
}

impl Iterator for DirectionIterator {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterate_forward() {
            Some(()) => Some(self.current_value.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections;
    use std::iter::FromIterator;

    macro_rules! direction_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (dimensions, expected_values) = $value;
                    let iterator = DirectionIterator::new(dimensions);
                    let result: collections::HashSet<_> = iterator.collect();
                    let expected = collections::HashSet::from_iter(expected_values.iter().cloned());
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    direction_tests! {
        one: (1, [vec![1]]),
        two: (2, [vec![1, 0], vec![1, 1], vec![0, 1], vec![-1, 1]]),
    }
}
