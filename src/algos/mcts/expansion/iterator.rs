use super::items;
use crate::rulesets;
use crate::rulesets::PermutationIteratorTrait;
use crate::rulesets::PlyIteratorTrait;
use std::collections;

pub struct Expander<RuleSet: rulesets::Permutable> {
    ply_iterator: RuleSet::PlyIterator,
    seen: collections::HashSet<RuleSet::State>,
}

impl<RuleSet: rulesets::Permutable> Expander<RuleSet> {
    pub fn new(ruleset: &RuleSet, current_state: RuleSet::State) -> Expander<RuleSet> {
        let ply_iterator = RuleSet::PlyIterator::new(ruleset, current_state);
        Expander {
            ply_iterator,
            seen: collections::HashSet::new(),
        }
    }

    pub fn iterate(&mut self, ruleset: &RuleSet) -> Option<items::Play<RuleSet>> {
        while let Some((ply, resulting_state)) = self.ply_iterator.next_state(&ruleset) {
            let permutations = RuleSet::PermutationIterator::new(&ruleset);
            let witness_state = permutations
                .map(|permutation| ruleset.swap_state(&resulting_state, &permutation))
                .min()
                .unwrap();
            if self.seen.contains(&witness_state) {
                continue;
            }
            self.seen.insert(witness_state);
            let status = ruleset.status(&resulting_state);
            return Some(items::Play {
                ply,
                state: resulting_state,
                status,
            });
        }
        None
    }
}
