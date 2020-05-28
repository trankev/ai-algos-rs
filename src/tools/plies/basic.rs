use crate::interface::rulesets;
use crate::interface::rulesets::PlyIteratorTrait;

pub struct BasicIterator<'a, RuleSet: rulesets::RuleSetTrait> {
    ruleset: &'a RuleSet,
    state: &'a RuleSet::State,
    iterator: RuleSet::PlyIterator,
}

impl<'a, RuleSet: rulesets::RuleSetTrait> BasicIterator<'a, RuleSet> {
    pub fn new(ruleset: &'a RuleSet, state: &'a RuleSet::State) -> BasicIterator<'a, RuleSet> {
        BasicIterator {
            ruleset,
            state,
            iterator: RuleSet::PlyIterator::new(ruleset, state),
        }
    }
}

impl<'a, RuleSet: rulesets::RuleSetTrait> Iterator for BasicIterator<'a, RuleSet> {
    type Item = RuleSet::Ply;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.iterate(self.ruleset, self.state)
    }
}
