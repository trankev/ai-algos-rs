use crate::interface::ai;
use crate::interface::rulesets;
use rand;
use rand::distributions;
use rand::distributions::Distribution;
use rand::rngs;
use std::error;
use std::marker;

pub struct Stochastic<'a, RuleSet, Policy>
where
    RuleSet: rulesets::RuleSetTrait,
    Policy: ai::Policy<RuleSet>,
{
    policy: &'a mut Policy,
    ruleset: marker::PhantomData<RuleSet>,
    rng: rngs::ThreadRng,
}

impl<'a, RuleSet, Policy> Stochastic<'a, RuleSet, Policy>
where
    RuleSet: rulesets::RuleSetTrait,
    Policy: ai::Policy<RuleSet>,
{
    pub fn new(policy: &'a mut Policy) -> Stochastic<'a, RuleSet, Policy> {
        Stochastic {
            policy,
            ruleset: marker::PhantomData,
            rng: rand::thread_rng(),
        }
    }
}

impl<'a, RuleSet, Policy> ai::Agent<RuleSet> for Stochastic<'a, RuleSet, Policy>
where
    RuleSet: rulesets::RuleSetTrait,
    Policy: ai::Policy<RuleSet>,
{
    fn play(&mut self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>> {
        let prediction = self.policy.predict(&state)?;
        let weight_index = distributions::WeightedIndex::new(
            prediction.probabilities.iter().map(|(_, prob)| prob),
        )?;
        let ply = prediction.probabilities[weight_index.sample(&mut self.rng)].0;
        Ok(ply)
    }
}
