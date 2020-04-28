use crate::utils::grids::symmetries;
use itertools::structs;
use itertools::Itertools;
use std::ops;

pub struct Symmetries {
    dimensions: Vec<isize>,
    pub destination: Vec<bool>,
    pub permutation: Vec<isize>,
    permutations: structs::Permutations<ops::Range<isize>>,
}

impl Symmetries {
    pub fn new(dimensions: Vec<isize>) -> Symmetries {
        let dimensions_size = dimensions.len();
        Symmetries {
            dimensions,
            destination: vec![true; dimensions_size],
            permutation: (0..dimensions_size as isize).collect(),
            permutations: (0..dimensions_size as isize).permutations(dimensions_size),
        }
    }

    fn iterate_destinations(&mut self) -> Option<()> {
        for value in self.destination.iter_mut() {
            *value = !*value;
            if *value {
                return Some(());
            }
        }
        None
    }

    fn iterate_permutations(&mut self) -> Option<()> {
        while let Some(permutation) = self.permutations.next() {
            if permutation
                .iter()
                .enumerate()
                .all(|(index, &dest)| self.dimensions[index] == self.dimensions[dest as usize])
            {
                self.permutation = permutation;
                return Some(());
            }
        }
        None
    }

    fn iterate(&mut self) -> Option<()> {
        if self.iterate_destinations().is_some() {
            return Some(());
        }
        if self.iterate_permutations().is_some() {
            self.destination = vec![false; self.dimensions.len()];
            return Some(());
        }
        None
    }
}

impl Iterator for Symmetries {
    type Item = symmetries::Symmetry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterate() {
            Some(()) => Some(symmetries::Symmetry {
                destination: self.destination.clone(),
                permutation: self.permutation.clone(),
            }),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections;

    #[test]
    fn test_symmetries() {
        let symmetries = Symmetries::new(vec![3, 3]);
        let result = symmetries.collect::<collections::HashSet<_>>();
        let expected = [
            symmetries::Symmetry {
                destination: vec![false, false],
                permutation: vec![0, 1],
            },
            symmetries::Symmetry {
                destination: vec![false, false],
                permutation: vec![1, 0],
            },
            symmetries::Symmetry {
                destination: vec![false, true],
                permutation: vec![0, 1],
            },
            symmetries::Symmetry {
                destination: vec![false, true],
                permutation: vec![1, 0],
            },
            symmetries::Symmetry {
                destination: vec![true, false],
                permutation: vec![0, 1],
            },
            symmetries::Symmetry {
                destination: vec![true, false],
                permutation: vec![1, 0],
            },
            symmetries::Symmetry {
                destination: vec![true, true],
                permutation: vec![0, 1],
            },
            symmetries::Symmetry {
                destination: vec![true, true],
                permutation: vec![1, 0],
            },
        ]
        .iter()
        .cloned()
        .collect::<collections::HashSet<_>>();
        assert_eq!(result, expected);
    }
}
