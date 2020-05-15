use crate::algos;
use crate::interface;

pub enum Response<RuleSet: interface::RuleSetTrait> {
    PlyConsiderations(Vec<algos::PlyConsideration<RuleSet::Ply>>),
}
