use super::variants;
use crate::interface;
use crate::rulesets::reversi;

pub struct SymmetryIterator {
    symmetry_count: usize,
    switched_player: bool,
}

impl<Variant: variants::BaseVariant> interface::SymmetryIteratorTrait<reversi::Reversi<Variant>>
    for SymmetryIterator
{
    fn new(ruleset: &reversi::Reversi<Variant>) -> Self {
        SymmetryIterator {
            symmetry_count: ruleset.grid_symmetry_count(),
            switched_player: true,
        }
    }
}

impl Iterator for SymmetryIterator {
    type Item = reversi::Symmetry;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.switched_player {
            self.switched_player = true;
            return Some(reversi::Symmetry {
                grid_symmetry_index: self.symmetry_count as u8,
                switched_players: true,
            });
        }
        if self.symmetry_count > 0 {
            self.symmetry_count -= 1;
            self.switched_player = false;
            return Some(reversi::Symmetry {
                grid_symmetry_index: self.symmetry_count as u8,
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
    use crate::rulesets::reversi;
    use std::collections;
    use std::iter;

    #[test]
    fn test_symmetrys() {
        let ruleset = reversi::Reversi::<reversi::Mini>::new();
        let iterator = SymmetryIterator::new(&ruleset);
        let result = iterator.collect::<collections::HashSet<_>>();
        let expected = (0u8..8)
            .flat_map(|index| {
                iter::once(reversi::Symmetry {
                    grid_symmetry_index: index,
                    switched_players: false,
                })
                .chain(iter::once(reversi::Symmetry {
                    grid_symmetry_index: index,
                    switched_players: true,
                }))
            })
            .collect::<collections::HashSet<_>>();
        assert_eq!(result, expected);
    }
}
