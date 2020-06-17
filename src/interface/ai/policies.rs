use crate::interface::ai;
use crate::interface::rulesets;
use std::error;

#[derive(Debug)]
pub struct Prediction<RuleSet: rulesets::RuleSetTrait> {
    pub value: f32,
    pub probabilities: Vec<(RuleSet::Ply, f32)>,
}

pub trait Policy<RuleSet: rulesets::RuleSetTrait> {
    fn predict(
        &mut self,
        state: &RuleSet::State,
    ) -> Result<Prediction<RuleSet>, Box<dyn error::Error>>;
}

pub trait Teachable<RuleSet: rulesets::RuleSetTrait>: Policy<RuleSet> {
    fn learn(&mut self, logs: &Vec<ai::PolicyLog<RuleSet>>) -> Result<(), Box<dyn error::Error>>;
}
