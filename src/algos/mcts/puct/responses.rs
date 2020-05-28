use crate::algos;
use crate::interface::rulesets;

pub enum Response<RuleSet: rulesets::RuleSetTrait> {
    PlyConsiderations(Vec<algos::PlyConsideration<RuleSet::Ply>>),
}
