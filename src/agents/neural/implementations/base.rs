use crate::interface::rulesets;

pub trait Implementation<RuleSet: rulesets::RuleSetTrait> {
    const DIMENSIONS: &'static [usize];

    fn encode_state(state: &RuleSet::State) -> Vec<f32>;
    fn decode_ply(ply_index: usize) -> RuleSet::Ply;
    fn encode_ply(ply: &RuleSet::Ply) -> usize;
}
