use crate::interface::ai;
use crate::interface::rulesets;
use crate::tools::plies;
use rand;
use rand::rngs;
use rand::seq::IteratorRandom;
use std::error;

pub struct Random<'a, RuleSet: rulesets::RuleSetTrait> {
    ruleset: &'a RuleSet,
    rng: rngs::ThreadRng,
}

impl<'a, RuleSet: rulesets::RuleSetTrait> Random<'a, RuleSet> {
    pub fn new(ruleset: &'a RuleSet) -> Random<'a, RuleSet> {
        Random {
            ruleset,
            rng: rand::thread_rng(),
        }
    }
}

impl<'a, RuleSet: rulesets::RuleSetTrait> ai::Agent<RuleSet> for Random<'a, RuleSet> {
    fn play(&mut self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>> {
        let available_plies = plies::BasicIterator::new(self.ruleset, state);
        let ply = available_plies.choose(&mut self.rng).unwrap();
        Ok(ply)
    }
}
