use super::items;
use crate::rulesets;
use crate::rulesets::PermutationIteratorTrait;
use crate::rulesets::PlyIteratorTrait;
use std::collections;
use std::rc;

pub struct Expander<RuleSet: rulesets::Permutable> {
    ply_iterator: RuleSet::PlyIterator,
    current_state: rc::Rc<RuleSet::State>,
    seen: collections::HashSet<RuleSet::State>,
}

impl<RuleSet: rulesets::Permutable> Expander<RuleSet> {
    pub fn new(current_state: rc::Rc<RuleSet::State>) -> Expander<RuleSet> {
        let ply_iterator = RuleSet::PlyIterator::new(current_state.clone());
        Expander {
            current_state,
            ply_iterator,
            seen: collections::HashSet::new(),
        }
    }

    pub fn iterate(&mut self, ruleset: &RuleSet) -> Option<items::PlyAndState<RuleSet>> {
        for ply in self.ply_iterator.by_ref() {
            let resulting_state = rc::Rc::new(ruleset.play(&self.current_state, &ply).unwrap());
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
