use super::ruleset;

pub trait PlyIteratorTrait<RuleSet: ruleset::RuleSetTrait> {
    fn new(ruleset: &RuleSet, state: &RuleSet::State) -> Self;
    fn iterate(&mut self, ruleset: &RuleSet, state: &RuleSet::State) -> Option<RuleSet::Ply>;
}
