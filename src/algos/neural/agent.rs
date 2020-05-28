use super::memory;
use super::network;
use crate::interface;
use crate::interface::TurnByTurnState;
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
    memory: memory::Memory,
}

impl<'a, RuleSet> Agent<'a, RuleSet>
where
    RuleSet: interface::EncodableState + interface::HasStatesWithSymmetries,
    RuleSet::State: Eq + interface::TurnByTurnState,
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
        Ok(Agent {
            ruleset,
            network,
            memory: memory::Memory::new(),
        })
    }

    pub fn play(&mut self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>> {
        let encoded_state = self.ruleset.encode_state(state);
        let mut allowed_plies = vec![0.0; RuleSet::PLY_COUNT];
        for ply in plies::SymmetriesIterator::new(self.ruleset, state) {
            let index = self.ruleset.encode_ply(&ply) as usize;
            allowed_plies[index] = 1.0;
        }
        let encoded_ply = self.network.play(&encoded_state, &allowed_plies)?;
        let ply = self.ruleset.decode_ply(encoded_ply as usize);
        self.memory.play(
            state.current_player(),
            &encoded_state,
            &allowed_plies,
            encoded_ply,
        );
        Ok(ply)
    }

    pub fn get_probabilities(
        &self,
        state: &RuleSet::State,
    ) -> Result<Vec<(RuleSet::Ply, f32)>, Box<dyn error::Error>> {
        let encoded_state = self.ruleset.encode_state(state);
        let mut allowed_plies = vec![0.0; RuleSet::PLY_COUNT];
        for ply in plies::SymmetriesIterator::new(self.ruleset, state) {
            let index = self.ruleset.encode_ply(&ply) as usize;
            allowed_plies[index] = 1.0;
        }
        let probabilities = self
            .network
            .get_probabilities(&encoded_state, &allowed_plies)?;
        let ply_values = probabilities
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                if *value == 0.0 {
                    None
                } else {
                    Some((self.ruleset.decode_ply(index), *value))
                }
            })
            .collect();

        Ok(ply_values)
    }

    pub fn learn(
        &mut self,
        status: interface::Status,
        discount_factor: f32,
    ) -> Result<(), Box<dyn error::Error>> {
        let rewards = self.memory.compute_rewards(status, discount_factor);
        self.network.learn(
            &self.memory.states,
            &self.memory.allowed_plies,
            &self.memory.actions,
            &rewards,
        )?;
        self.memory.clear();
        Ok(())
    }

    pub fn save(&self, path: String) -> Result<(), Box<dyn error::Error>> {
        self.network.save(path)
    }
}
