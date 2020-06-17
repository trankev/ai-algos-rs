use crate::interface::ai;
use crate::interface::rulesets;

pub struct PolicyTurn<RuleSet: rulesets::RuleSetTrait> {
    pub state: RuleSet::State,
    pub prediction: ai::Prediction<RuleSet>,
}

pub struct PolicyLog<RuleSet: rulesets::RuleSetTrait> {
    pub history: Vec<PolicyTurn<RuleSet>>,
    pub status: rulesets::Status,
}
