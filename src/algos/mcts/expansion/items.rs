use crate::rulesets;

pub struct PlyAndState<RuleSet: rulesets::RuleSetTrait> {
    pub ply: RuleSet::Ply,
    pub state: RuleSet::State,
}
