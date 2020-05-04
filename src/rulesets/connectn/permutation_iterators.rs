use super::variants;
use crate::rulesets;
use crate::rulesets::connectn;
use crate::utils::bitarray;
use std::ops;

pub struct PermutationIterator {
    permutation_count: usize,
    switched_player: bool,
}

impl<ArrayType, Variant> rulesets::PermutationIteratorTrait<connectn::RuleSet<ArrayType, Variant>>
    for PermutationIterator
where
    Variant: variants::BaseVariant,
    ArrayType: bitarray::BitArray,
    for<'a> ArrayType: ops::BitAnd<&'a ArrayType, Output = ArrayType>
        + ops::BitOr<&'a ArrayType, Output = ArrayType>
        + ops::BitXor<&'a ArrayType, Output = ArrayType>,
    for<'a> &'a ArrayType: ops::BitAnd<ArrayType, Output = ArrayType>
        + ops::BitOr<ArrayType, Output = ArrayType>
        + ops::BitXor<ArrayType, Output = ArrayType>,
    for<'a, 'b> &'a ArrayType: ops::BitAnd<&'b ArrayType, Output = ArrayType>
        + ops::BitOr<&'b ArrayType, Output = ArrayType>
        + ops::BitXor<&'b ArrayType, Output = ArrayType>,
{
    fn new(ruleset: &connectn::RuleSet<ArrayType, Variant>) -> Self {
        PermutationIterator {
            permutation_count: ruleset.grid_symmetry_count(),
            switched_player: true,
        }
    }
}

impl Iterator for PermutationIterator {
    type Item = connectn::Permutation;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.switched_player {
            self.switched_player = true;
            return Some(connectn::Permutation {
                grid_permutation_index: self.permutation_count as u8,
                switched_players: true,
            });
        }
        if self.permutation_count > 0 {
            self.permutation_count -= 1;
            self.switched_player = false;
            return Some(connectn::Permutation {
                grid_permutation_index: self.permutation_count as u8,
                switched_players: false,
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rulesets::connectn;
    use crate::rulesets::PermutationIteratorTrait;
    use std::collections;
    use std::iter;

    #[test]
    fn test_permutations() {
        let ruleset = connectn::TicTacToe::new();
        let iterator = PermutationIterator::new(&ruleset);
        let result = iterator.collect::<collections::HashSet<_>>();
        let expected = (0u8..8)
            .flat_map(|index| {
                iter::once(connectn::Permutation {
                    grid_permutation_index: index,
                    switched_players: false,
                })
                .chain(iter::once(connectn::Permutation {
                    grid_permutation_index: index,
                    switched_players: true,
                }))
            })
            .collect::<collections::HashSet<_>>();
        assert_eq!(result, expected);
    }
}
