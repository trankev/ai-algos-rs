use super::variants;
use crate::interface;
use crate::rulesets::connectn;

pub struct PermutationIterator {
    permutation_count: usize,
    switched_player: bool,
}

impl<Variant: variants::BaseVariant> interface::PermutationIteratorTrait<connectn::RuleSet<Variant>>
    for PermutationIterator
{
    fn new(ruleset: &connectn::RuleSet<Variant>) -> Self {
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
    use crate::interface::PermutationIteratorTrait;
    use crate::rulesets::connectn;
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
