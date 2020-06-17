use super::implementations;
use crate::interface::rulesets;

pub fn encode<RuleSet, Implementation>(predictions: &Vec<(RuleSet::Ply, f32)>) -> Vec<f32>
where
    RuleSet: rulesets::RuleSetTrait,
    Implementation: implementations::Implementation<RuleSet>,
{
    let mut result = vec![0.0; Implementation::PLY_COUNT];
    for (ply, prob) in predictions {
        let ply_index = Implementation::encode_ply(ply);
        result[ply_index] = *prob;
    }
    result
}
