use crate::interface::rulesets;
use std::error;

pub trait QValue<RuleSet: rulesets::RuleSetTrait> {
    fn evaluate(&mut self, state: &RuleSet::State) -> Result<f32, Box<dyn error::Error>>;
}
