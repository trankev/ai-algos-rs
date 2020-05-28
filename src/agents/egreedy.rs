use crate::interface::ai;
use crate::interface::rulesets;
use crate::tools::plies;
use rand;
use rand::rngs;
use rand::seq::IteratorRandom;
use rand::Rng;
use std::error;

pub struct EGreedy<'a, RuleSet, Agent>
where
    RuleSet: rulesets::RuleSetTrait,
    Agent: ai::Policy<RuleSet>,
{
    ruleset: &'a RuleSet,
    exploration_rate: f32,
    policy: &'a mut Agent,
    rng: rngs::ThreadRng,
}

impl<'a, RuleSet, Agent> EGreedy<'a, RuleSet, Agent>
where
    RuleSet: rulesets::RuleSetTrait,
    Agent: ai::Policy<RuleSet>,
{
    pub fn new(
        ruleset: &'a RuleSet,
        exploration_rate: f32,
        policy: &'a mut Agent,
    ) -> EGreedy<'a, RuleSet, Agent> {
        let rng = rand::thread_rng();
        EGreedy {
            ruleset,
            exploration_rate,
            policy,
            rng,
        }
    }
}

impl<'a, RuleSet, Agent> ai::Policy<RuleSet> for EGreedy<'a, RuleSet, Agent>
where
    RuleSet: rulesets::RuleSetTrait,
    Agent: ai::Policy<RuleSet>,
{
    fn play(&mut self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>> {
        if self.rng.gen_range(0.0, 1.0) < self.exploration_rate {
            let available_plies = plies::BasicIterator::new(self.ruleset, state);
            let ply = available_plies.choose(&mut self.rng).unwrap();
            Ok(ply)
        } else {
            self.policy.play(state)
        }
    }
}
