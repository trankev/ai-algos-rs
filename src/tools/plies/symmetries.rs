use crate::interface::rulesets;
use crate::interface::rulesets::PlyIteratorTrait;
use crate::interface::rulesets::SymmetryIteratorTrait;
use std::collections;
use std::hash;

pub struct SymmetriesIterator<'a, RuleSet: rulesets::HasStatesWithSymmetries>
where
    RuleSet::Ply: Eq + Ord + hash::Hash,
    RuleSet::State: Eq,
{
    ruleset: &'a RuleSet,
    state: &'a RuleSet::State,
    iterator: RuleSet::PlyIterator,
    permutations: Vec<RuleSet::Symmetry>,
    seen: collections::HashSet<RuleSet::Ply>,
}

impl<'a, RuleSet: rulesets::HasStatesWithSymmetries> SymmetriesIterator<'a, RuleSet>
where
    RuleSet::Ply: Eq + Ord + hash::Hash,
    RuleSet::State: Eq,
{
    pub fn new(ruleset: &'a RuleSet, state: &'a RuleSet::State) -> SymmetriesIterator<'a, RuleSet> {
        let iterator = RuleSet::SymmetryIterator::new(ruleset);
        let permutations = iterator
            .filter(|permutation| ruleset.swap_state(state, permutation) == *state)
            .collect();
        SymmetriesIterator {
            ruleset,
            state,
            iterator: RuleSet::PlyIterator::new(ruleset, state),
            permutations,
            seen: collections::HashSet::new(),
        }
    }
}

impl<'a, RuleSet: rulesets::HasStatesWithSymmetries> Iterator for SymmetriesIterator<'a, RuleSet>
where
    RuleSet::Ply: Eq + Ord + hash::Hash,
    RuleSet::State: Eq,
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
    use crate::interface::rulesets::RuleSetTrait;
    use crate::rulesets::connectn;

    #[test]
    fn test_mini_reversi() {
        let ruleset = connectn::TicTacToe::new();
        let state = ruleset.initial_state();
        let iterator = SymmetriesIterator::new(&ruleset, &state);
        let mut result = iterator.collect::<Vec<_>>();
        result.sort();
        let expected = vec![
            connectn::Ply::new(0),
            connectn::Ply::new(1),
            connectn::Ply::new(4),
        ];
        assert_eq!(result, expected);
    }
}
