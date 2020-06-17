use crate::interface::ai;
use crate::interface::rulesets;
use std::error;
use std::path;

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

pub trait WithMemory {
    fn save<P: AsRef<path::Path>>(&self, project_folder: P) -> Result<(), Box<dyn error::Error>>;

    fn load<P: AsRef<path::Path>>(
        &mut self,
        project_folder: P,
    ) -> Result<(), Box<dyn error::Error>>;
}
