use super::implementations;
use super::networks;
use crate::interface::ai;
use crate::interface::rulesets;
use std::error;
use std::marker;
use std::path;

pub struct Agent<'a, RuleSet, Implementation>
where
    RuleSet: rulesets::RuleSetTrait,
    Implementation: implementations::Implementation<RuleSet>,
{
    ruleset: &'a RuleSet,
    network: networks::Network,
    implementation: marker::PhantomData<Implementation>,
}

impl<'a, RuleSet, Implementation> Agent<'a, RuleSet, Implementation>
where
    RuleSet: rulesets::RuleSetTrait,
    Implementation: implementations::Implementation<RuleSet>,
{
    pub fn new<P: AsRef<path::Path>>(
        ruleset: &'a RuleSet,
        project_folder: P,
    ) -> Result<Agent<RuleSet, Implementation>, Box<dyn error::Error>> {
        let network = networks::Network::new(project_folder, Implementation::DIMENSIONS)?;
        network.initialize()?;
        let implementation = marker::PhantomData;
        let agent = Agent {
            ruleset,
            network,
            implementation,
        };
        Ok(agent)
    }
}

impl<'a, RuleSet, Implementation> ai::Policy<RuleSet> for Agent<'a, RuleSet, Implementation>
where
    RuleSet: rulesets::RuleSetTrait,
    Implementation: implementations::Implementation<RuleSet>,
{
    fn predict(
        &mut self,
        state: &RuleSet::State,
    ) -> Result<ai::Prediction<RuleSet>, Box<dyn error::Error>> {
        let encoded_state = Implementation::encode_state(state);
        let (value, raw_probs) = self.network.predict(&encoded_state)?;
        let probabilities = raw_probs
            .iter()
            .enumerate()
            .map(|(index, probability)| (Implementation::decode_ply(index), *probability))
            .collect();
        let result = ai::Prediction {
            value,
            probabilities,
        };
        Ok(result)
    }
}
