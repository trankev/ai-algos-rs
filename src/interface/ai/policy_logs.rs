use crate::interface::ai;
use crate::interface::rulesets;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PolicyTurn<RuleSet: rulesets::RuleSetTrait> {
    pub state: RuleSet::State,
    pub prediction: ai::Prediction<RuleSet>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PolicyLog<RuleSet: rulesets::RuleSetTrait> {
    pub history: Vec<PolicyTurn<RuleSet>>,
    pub status: rulesets::Status,
}
