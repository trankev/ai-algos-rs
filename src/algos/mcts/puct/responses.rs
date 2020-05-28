use crate::interface::ai;
use crate::interface::rulesets;

pub enum Response<RuleSet: rulesets::RuleSetTrait> {
    PlyConsiderations(Vec<ai::PlyConsideration<RuleSet::Ply>>),
}
