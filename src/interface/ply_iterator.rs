use super::ruleset;

pub trait PlyIteratorTrait<RuleSet: ruleset::RuleSetTrait>: Iterator<Item = RuleSet::Ply> {
    fn new(ruleset: &RuleSet, state: RuleSet::State) -> Self;
    fn current_state(&self) -> &RuleSet::State;

    fn next_state(&mut self, ruleset: &RuleSet) -> Option<(RuleSet::Ply, RuleSet::State)> {
        match self.next() {
            Some(ply) => Some((ply, ruleset.play(&self.current_state(), &ply).unwrap())),
            None => None,
        }
    }
}
