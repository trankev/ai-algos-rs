use crate::interface;
use crate::interface::PlyIteratorTrait;

pub struct PlyIterator<'a, RuleSet: interface::RuleSetTrait> {
    ruleset: &'a RuleSet,
    state: &'a RuleSet::State,
    iterator: RuleSet::PlyIterator,
}

impl<'a, RuleSet: interface::RuleSetTrait> PlyIterator<'a, RuleSet> {
    pub fn new(ruleset: &'a RuleSet, state: &'a RuleSet::State) -> PlyIterator<'a, RuleSet> {
        PlyIterator {
            ruleset,
            state,
            iterator: RuleSet::PlyIterator::new(ruleset, state),
        }
    }
}

impl<'a, RuleSet: interface::RuleSetTrait> Iterator for PlyIterator<'a, RuleSet> {
    type Item = RuleSet::Ply;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.iterate(self.ruleset, self.state)
    }
}
