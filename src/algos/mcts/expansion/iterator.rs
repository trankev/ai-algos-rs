use super::items;
use crate::interface;
use crate::interface::PermutationIteratorTrait;
use crate::interface::PlyIteratorTrait;
use std::collections;

pub struct Expander<'a, RuleSet: interface::Permutable> {
    ply_iterator: RuleSet::PlyIterator,
    seen: collections::HashSet<RuleSet::State>,
    ruleset: &'a RuleSet,
    state: &'a RuleSet::State,
}

impl<'a, RuleSet: interface::Permutable> Expander<'a, RuleSet> {
    pub fn new(ruleset: &'a RuleSet, state: &'a RuleSet::State) -> Expander<'a, RuleSet> {
        let ply_iterator = RuleSet::PlyIterator::new(ruleset, state);
        Expander {
            ply_iterator,
            seen: collections::HashSet::new(),
            ruleset,
            state,
        }
    }

    pub fn iterate(&mut self) -> Option<items::Play<RuleSet>> {
        while let Some(ply) = self.ply_iterator.iterate(&self.ruleset, &self.state) {
            let resulting_state = self.ruleset.play(self.state, &ply).unwrap();
            let permutations = RuleSet::PermutationIterator::new(&self.ruleset);
            let witness_state = permutations
                .map(|permutation| self.ruleset.swap_state(&resulting_state, &permutation))
                .min()
                .unwrap();
            if self.seen.contains(&witness_state) {
                continue;
            }
            self.seen.insert(witness_state);
            let status = self.ruleset.status(&resulting_state);
            return Some(items::Play {
                ply,
                state: resulting_state,
                status,
            });
        }
        None
    }
}
