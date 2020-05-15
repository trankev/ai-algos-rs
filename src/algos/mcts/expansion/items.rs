use crate::interface;

pub struct Play<RuleSet: interface::RuleSetTrait> {
    pub ply: RuleSet::Ply,
    pub state: RuleSet::State,
    pub status: interface::Status,
}
