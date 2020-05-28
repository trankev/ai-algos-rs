use crate::interface::rulesets;
use std::error;

pub trait Policy<RuleSet: rulesets::RuleSetTrait> {
    fn play(&mut self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>>;
}
