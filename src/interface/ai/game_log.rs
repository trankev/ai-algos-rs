use crate::interface::rulesets;

#[derive(Debug)]
pub struct GameLog<RuleSet: rulesets::RuleSetTrait> {
    pub history: Vec<(RuleSet::State, RuleSet::Ply)>,
    pub status: rulesets::Status,
}

impl<RuleSet: rulesets::RuleSetTrait> GameLog<RuleSet> {
    pub fn new() -> GameLog<RuleSet> {
        GameLog {
            history: Vec::new(),
            status: rulesets::Status::Ongoing,
        }
    }
}
