use super::variants;
use crate::interface;
use crate::rulesets::connectn;

pub struct SymmetryIterator {
    permutation_count: usize,
    switched_player: bool,
}

impl<Variant: variants::BaseVariant> interface::SymmetryIteratorTrait<connectn::RuleSet<Variant>>
    for SymmetryIterator
{
    fn new(ruleset: &connectn::RuleSet<Variant>) -> Self {
        SymmetryIterator {
            permutation_count: ruleset.grid_symmetry_count(),
            switched_player: true,
        }
    }
}

impl Iterator for SymmetryIterator {
    type Item = connectn::Symmetry;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.switched_player {
            self.switched_player = true;
            return Some(connectn::Symmetry {
                grid_permutation_index: self.permutation_count as u8,
                switched_players: true,
            });
        }
        if self.permutation_count > 0 {
            self.permutation_count -= 1;
            self.switched_player = false;
            return Some(connectn::Symmetry {
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
    use crate::interface::SymmetryIteratorTrait;
    use crate::rulesets::connectn;
    use std::collections;
    use std::iter;

    #[test]
    fn test_permutations() {
        let ruleset = connectn::TicTacToe::new();
        let iterator = SymmetryIterator::new(&ruleset);
        let result = iterator.collect::<collections::HashSet<_>>();
        let expected = (0u8..8)
            .flat_map(|index| {
                iter::once(connectn::Symmetry {
                    grid_permutation_index: index,
                    switched_players: false,
                })
                .chain(iter::once(connectn::Symmetry {
                    grid_permutation_index: index,
                    switched_players: true,
                }))
            })
            .collect::<collections::HashSet<_>>();
        assert_eq!(result, expected);
    }
}
