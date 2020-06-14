use crate::interface::rulesets;

pub struct Play<RuleSet: rulesets::RuleSetTrait> {
    pub ply: RuleSet::Ply,
    pub state: RuleSet::State,
    pub status: rulesets::Status,
    pub current_player: rulesets::Player,
}
