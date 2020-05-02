use super::conversion;
use super::iterator;
use crate::utils::grids;
use crate::utils::grids::positions;
use crate::utils::vectors;

#[derive(Debug)]
pub struct SymmetryTable {
    pub permutations: Vec<Vec<usize>>,
    pub reverses: Vec<usize>,
}

impl SymmetryTable {
    pub fn new(dimensions: &Vec<isize>) -> SymmetryTable {
        let symmetries = iterator::Symmetries::new(dimensions.clone());
        let strides = grids::compute_strides(&dimensions);
        let permutations: Vec<Vec<usize>> = symmetries
            .map(|symmetry| {
                positions::Positions::new(dimensions.clone())
                    .map(|position| {
                        let permuted = conversion::convert_position(
                            &dimensions,
                            &position,
                            &symmetry.destination,
                            &symmetry.permutation,
                        );
                        vectors::dot_product(&permuted, &strides) as usize
                    })
                    .collect()
            })
            .collect();
        let reverses = permutations
            .iter()
            .map(|permutation| {
                let reverse = revert_permutation_indices(permutation);
                permutations
                    .iter()
                    .position(|item| item == &reverse)
                    .unwrap()
            })
            .collect();
        SymmetryTable {
            permutations,
            reverses,
        }
    }
}

fn revert_permutation_indices(indices: &Vec<usize>) -> Vec<usize> {
    let mut result = vec![0; indices.len()];
    indices
        .iter()
        .enumerate()
        .for_each(|(index, &value)| result[value] = index);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetry_table() {
        let table = SymmetryTable::new(&vec![3, 3]);
        let initial: Vec<usize> = (0..9).collect();
        for (index, permutation) in table.permutations.iter().enumerate() {
            let permuted = permutation
                .iter()
                .map(|&target| initial[target])
                .collect::<Vec<_>>();
            let reverse_permutation = &table.permutations[table.reverses[index]];
            let reverted = reverse_permutation
                .iter()
                .map(|&target| permuted[target])
                .collect::<Vec<_>>();
            assert_eq!(initial, reverted);
        }
    }
}
