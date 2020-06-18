use crate::interface::ai;
use crate::interface::rulesets;
use std::error;

pub trait Agent<RuleSet: rulesets::RuleSetTrait> {
    fn play(&mut self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>>;
}

pub trait Learner<RuleSet: rulesets::RuleSetTrait>: Agent<RuleSet> {
    type Metrics;

    fn learn(
        &mut self,
        game_logs: &[ai::GameLog<RuleSet>],
    ) -> Result<Self::Metrics, Box<dyn error::Error>>;
}
