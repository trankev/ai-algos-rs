use crate::interface::rulesets;

pub trait Implementation<RuleSet: rulesets::RuleSetTrait> {
    const STATE_DIMENSIONS: &'static [usize];
    const PLY_COUNT: usize;

    fn encode_state(state: &RuleSet::State) -> Vec<f32>;
    fn decode_ply(ply_index: usize) -> RuleSet::Ply;
    fn encode_ply(ply: &RuleSet::Ply) -> usize;
}
