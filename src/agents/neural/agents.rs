use super::implementations;
use super::networks;
use crate::interface::ai;
use crate::interface::rulesets;
use crate::tools::plies;
use std::error;
use std::hash;
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
    RuleSet: rulesets::HasStatesWithSymmetries,
    Implementation: implementations::Implementation<RuleSet>,
    RuleSet::Ply: Ord + hash::Hash,
    RuleSet::State: Eq,
{
    pub fn new<P: AsRef<path::Path>>(
        ruleset: &'a RuleSet,
        project_folder: P,
    ) -> Result<Agent<RuleSet, Implementation>, Box<dyn error::Error>> {
        let network = networks::Network::new(
            project_folder,
            Implementation::STATE_DIMENSIONS,
            Implementation::PLY_COUNT,
        )?;
        network.initialize()?;
        let implementation = marker::PhantomData;
        let agent = Agent {
            ruleset,
            network,
            implementation,
        };
        Ok(agent)
    }

    fn compute_allowed_plies(&self, state: &RuleSet::State) -> Vec<f32> {
        let mut allowed_plies = vec![0.0; Implementation::PLY_COUNT];
        for ply in plies::SymmetriesIterator::new(self.ruleset, state) {
            let index = Implementation::encode_ply(&ply) as usize;
            allowed_plies[index] = 1.0;
        }
        allowed_plies
    }
}

impl<'a, RuleSet, Implementation> ai::Policy<RuleSet> for Agent<'a, RuleSet, Implementation>
where
    RuleSet: rulesets::HasStatesWithSymmetries,
    Implementation: implementations::Implementation<RuleSet>,
    RuleSet::Ply: Ord + hash::Hash,
    RuleSet::State: Eq,
{
    fn predict(
        &mut self,
        state: &RuleSet::State,
    ) -> Result<ai::Prediction<RuleSet>, Box<dyn error::Error>> {
        let encoded_state = Implementation::encode_state(state);
        let allowed_plies = self.compute_allowed_plies(state);
        let (value, raw_probs) = self.network.predict(&encoded_state, &allowed_plies)?;
        let probabilities = raw_probs
            .iter()
            .enumerate()
            .filter_map(|(index, probability)| {
                if *probability == 0.0 {
                    None
                } else {
                    Some((Implementation::decode_ply(index), *probability))
                }
            })
            .collect();
        let result = ai::Prediction {
            value,
            probabilities,
        };
        Ok(result)
    }
}
