use crate::interface::ai;
use crate::interface::rulesets;
use std::error;

pub trait Policy<RuleSet: rulesets::RuleSetTrait> {
    fn play(&mut self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>>;
}

pub trait Teachable<RuleSet: rulesets::RuleSetTrait> {
    fn learn(&mut self, game_logs: &Vec<ai::GameLog<RuleSet>>)
        -> Result<(), Box<dyn error::Error>>;
}
