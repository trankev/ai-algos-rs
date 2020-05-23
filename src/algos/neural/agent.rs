use super::network;
use crate::interface;
use std::error;
use std::path;

pub struct Agent<'a, RuleSet: interface::EncodableState> {
    network: network::Network,
    ruleset: &'a RuleSet,
}

impl<'a, RuleSet: interface::EncodableState> Agent<'a, RuleSet> {
    pub fn new<P: AsRef<path::Path>>(
        ruleset: &'a RuleSet,
        model_file: P,
        data_folder: Option<String>,
    ) -> Result<Agent<RuleSet>, Box<dyn error::Error>> {
        let network = network::Network::from_file(
            model_file,
            RuleSet::STATE_SIZE as u64,
            RuleSet::PLY_COUNT,
        )?;
        match data_folder {
            Some(folder) => network.load(folder)?,
            None => network.initialize()?,
        }
        Ok(Agent { ruleset, network })
    }

    pub fn play(&self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>> {
        let encoded_state = self.ruleset.encode_state(state);
        let encoded_ply = self.network.play(&encoded_state)?;
        let ply = self.ruleset.decode_ply(encoded_ply);
        Ok(ply)
    }

    pub fn save(&self, path: String) -> Result<(), Box<dyn error::Error>> {
        self.network.save(path)
    }
}
