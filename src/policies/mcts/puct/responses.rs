use crate::interface::ai;
use crate::interface::rulesets;

pub struct Response<RuleSet: rulesets::RuleSetTrait> {
    pub considerations: Vec<ai::PlyConsideration<RuleSet::Ply>>,
}
