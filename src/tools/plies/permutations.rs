use crate::interface;
use crate::interface::PermutationIteratorTrait;
use crate::interface::PlyIteratorTrait;
use std::collections;

pub struct PermutationsIterator<'a, RuleSet: interface::WithPermutableState>
where
    RuleSet::Ply: interface::ComparablePly,
    RuleSet::State: interface::ComparableState,
{
    ruleset: &'a RuleSet,
    state: &'a RuleSet::State,
    iterator: RuleSet::PlyIterator,
    permutations: Vec<RuleSet::Permutation>,
    seen: collections::HashSet<RuleSet::Ply>,
}

impl<'a, RuleSet: interface::WithPermutableState> PermutationsIterator<'a, RuleSet>
where
    RuleSet::Ply: interface::ComparablePly,
    RuleSet::State: interface::ComparableState,
{
    pub fn new(
        ruleset: &'a RuleSet,
        state: &'a RuleSet::State,
    ) -> PermutationsIterator<'a, RuleSet> {
        let iterator = RuleSet::PermutationIterator::new(ruleset);
        let permutations = iterator
            .filter(|permutation| ruleset.swap_state(state, permutation) == *state)
            .collect();
        PermutationsIterator {
            ruleset,
            state,
            iterator: RuleSet::PlyIterator::new(ruleset, state),
            permutations,
            seen: collections::HashSet::new(),
        }
    }
}

impl<'a, RuleSet: interface::WithPermutableState> Iterator for PermutationsIterator<'a, RuleSet>
where
    RuleSet::Ply: interface::ComparablePly,
    RuleSet::State: interface::ComparableState,
{
    type Item = RuleSet::Ply;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ply) = self.iterator.iterate(self.ruleset, self.state) {
            let invariant_ply = self
                .permutations
                .iter()
                .map(|permutation| self.ruleset.swap_ply(&ply, permutation))
                .min()
                .unwrap();
            if self.seen.contains(&invariant_ply) {
                continue;
            }
            self.seen.insert(invariant_ply);
            return Some(invariant_ply);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::RuleSetTrait;
    use crate::interface::StateTrait;
    use crate::rulesets::connectn;

    #[test]
    fn test_mini_reversi() {
        let ruleset = connectn::TicTacToe::new();
        let state = ruleset.initial_state();
        println!("{}", state.ascii_representation());
        let iterator = PermutationsIterator::new(&ruleset, &state);
        let mut result = iterator.collect::<Vec<_>>();
        result.sort();
        let expected = vec![
            connectn::Ply { index: 0 },
            connectn::Ply { index: 1 },
            connectn::Ply { index: 4 },
        ];
        assert_eq!(result, expected);
    }
}
