use crate::algos;
use crate::rulesets;

pub enum Response<RuleSet: rulesets::RuleSetTrait> {
    PlyConsiderations(Vec<algos::PlyConsideration<RuleSet::Ply>>),
}
