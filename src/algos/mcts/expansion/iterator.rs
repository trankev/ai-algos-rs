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
    pub fn new(current_state: RuleSet::State) -> Expander<RuleSet> {
        let ply_iterator = RuleSet::PlyIterator::new(current_state);
        Expander {
            ply_iterator,
            seen: collections::HashSet::new(),
        }
    }

    pub fn iterate(&mut self, ruleset: &RuleSet) -> Option<items::PlyAndState<RuleSet>> {
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
            return Some(items::PlyAndState {
                ply,
                state: resulting_state,
            });
        }
        None
    }
}
