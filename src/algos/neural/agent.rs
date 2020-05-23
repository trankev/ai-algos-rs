use super::network;
use crate::interface;
use crate::tools::plies;
use std::error;
use std::hash;
use std::path;

pub struct Agent<'a, RuleSet>
where
    RuleSet: interface::EncodableState + interface::HasStatesWithSymmetries,
    RuleSet::State: Eq,
    RuleSet::Ply: Ord + hash::Hash,
{
    network: network::Network,
    ruleset: &'a RuleSet,
}

impl<'a, RuleSet> Agent<'a, RuleSet>
where
    RuleSet: interface::EncodableState + interface::HasStatesWithSymmetries,
    RuleSet::State: Eq,
    RuleSet::Ply: Ord + hash::Hash,
{
    pub fn new<P: AsRef<path::Path>>(
        ruleset: &'a RuleSet,
        model_file: P,
        data_folder: Option<String>,
    ) -> Result<Agent<RuleSet>, Box<dyn error::Error>> {
        let network = network::Network::from_file(
            model_file,
            RuleSet::STATE_SIZE as u64,
            RuleSet::PLY_COUNT as u64,
        )?;
        match data_folder {
            Some(folder) => network.load(folder)?,
            None => network.initialize()?,
        }
        Ok(Agent { ruleset, network })
    }

    pub fn play(&self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>> {
        let encoded_state = self.ruleset.encode_state(state);
        let mut allowed_plies = vec![0.0; RuleSet::PLY_COUNT];
        for ply in plies::SymmetriesIterator::new(self.ruleset, state) {
            let index = self.ruleset.encode_ply(&ply) as usize;
            allowed_plies[index] = 1.0;
        }
        let encoded_ply = self.network.play(&encoded_state, &allowed_plies)?;
        let ply = self.ruleset.decode_ply(encoded_ply);
        Ok(ply)
    }

    pub fn get_probabilities(
        &self,
        state: &RuleSet::State,
    ) -> Result<Vec<f32>, Box<dyn error::Error>> {
        let encoded_state = self.ruleset.encode_state(state);
        let mut allowed_plies = vec![0.0; RuleSet::PLY_COUNT];
        for ply in plies::SymmetriesIterator::new(self.ruleset, state) {
            let index = self.ruleset.encode_ply(&ply) as usize;
            allowed_plies[index] = 1.0;
        }
        let probabilities = self
            .network
            .get_probabilities(&encoded_state, &allowed_plies)?;
        Ok(probabilities)
    }

    pub fn save(&self, path: String) -> Result<(), Box<dyn error::Error>> {
        self.network.save(path)
    }
}
