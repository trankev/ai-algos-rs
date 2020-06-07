use super::actions;
use super::learning_metrics;
use super::network;
use super::replay_buffer;
use crate::interface::ai;
use crate::interface::rulesets;
use crate::interface::rulesets::TurnByTurnState;
use crate::tools::plies;
use std::error;
use std::hash;
use std::path;

pub struct Agent<'a, RuleSet>
where
    RuleSet: rulesets::EncodableState + rulesets::HasStatesWithSymmetries,
    RuleSet::State: Eq,
    RuleSet::Ply: Ord + hash::Hash,
{
    network: network::Network,
    ruleset: &'a RuleSet,
    discount_factor: f32,
}

impl<'a, RuleSet> Agent<'a, RuleSet>
where
    RuleSet: rulesets::EncodableState + rulesets::HasStatesWithSymmetries,
    RuleSet::State: Eq + rulesets::TurnByTurnState,
    RuleSet::Ply: Ord + hash::Hash,
{
    pub fn new<P: AsRef<path::Path>>(
        ruleset: &'a RuleSet,
        model_file: P,
        data_folder: Option<String>,
        discount_factor: f32,
        action_choosing: actions::ActionChoosing,
    ) -> Result<Agent<RuleSet>, Box<dyn error::Error>> {
        let network = network::Network::from_file(
            model_file,
            RuleSet::STATE_SIZE as u64,
            RuleSet::PLY_COUNT as u64,
            action_choosing,
        )?;
        match data_folder {
            Some(folder) => network.load(folder)?,
            None => network.initialize()?,
        }
        Ok(Agent {
            ruleset,
            network,
            discount_factor,
        })
    }

    fn compute_allowed_plies(&self, state: &RuleSet::State) -> Vec<f32> {
        let mut allowed_plies = vec![0.0; RuleSet::PLY_COUNT];
        for ply in plies::SymmetriesIterator::new(self.ruleset, state) {
            let index = self.ruleset.encode_ply(&ply) as usize;
            allowed_plies[index] = 1.0;
        }
        allowed_plies
    }

    pub fn get_probabilities(
        &self,
        state: &RuleSet::State,
    ) -> Result<Vec<(RuleSet::Ply, f32)>, Box<dyn error::Error>> {
        let encoded_state = self.ruleset.encode_state(state);
        let allowed_plies = self.compute_allowed_plies(state);
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

    fn build_buffer(&self, logs: &Vec<ai::GameLog<RuleSet>>) -> replay_buffer::ReplayBuffer {
        let mut result = replay_buffer::ReplayBuffer::new();
        for log in logs {
            let mut discount = 1.0;
            for (state, ply) in log.history.iter().rev() {
                let reward = match log.status.player_pov(&state.current_player()) {
                    rulesets::PlayerStatus::Win => 1.0,
                    rulesets::PlayerStatus::Draw => 0.0,
                    rulesets::PlayerStatus::Loss => -1.0,
                    rulesets::PlayerStatus::Ongoing => unreachable!(),
                };
                result.rewards.push((reward + 1.0) * discount / 2.0);
                result.qvalues.push(reward);
                result.states.extend(self.ruleset.encode_state(&state));
                result
                    .allowed_plies
                    .extend(self.compute_allowed_plies(&state));
                result.plies.push(self.ruleset.encode_ply(ply) as i32);
                discount *= self.discount_factor;
            }
        }
        result
    }

    pub fn save(&self, path: String) -> Result<(), Box<dyn error::Error>> {
        self.network.save(path)
    }
}

impl<'a, RuleSet> ai::Policy<RuleSet> for Agent<'a, RuleSet>
where
    RuleSet: rulesets::EncodableState + rulesets::HasStatesWithSymmetries,
    RuleSet::State: Eq + rulesets::TurnByTurnState,
    RuleSet::Ply: Ord + hash::Hash,
{
    fn play(&mut self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>> {
        let encoded_state = self.ruleset.encode_state(state);
        let allowed_plies = self.compute_allowed_plies(state);
        let encoded_ply = self.network.play(&encoded_state, &allowed_plies)?;
        let ply = self.ruleset.decode_ply(encoded_ply as usize);
        Ok(ply)
    }
}

impl<'a, RuleSet> ai::Teachable<RuleSet> for Agent<'a, RuleSet>
where
    RuleSet: rulesets::EncodableState + rulesets::HasStatesWithSymmetries,
    RuleSet::State: Eq + rulesets::TurnByTurnState,
    RuleSet::Ply: Ord + hash::Hash,
{
    type Metrics = learning_metrics::LearningMetrics;

    fn learn(
        &mut self,
        game_logs: &Vec<ai::GameLog<RuleSet>>,
    ) -> Result<learning_metrics::LearningMetrics, Box<dyn error::Error>> {
        let replay_buffer = self.build_buffer(game_logs);
        let metrics = self.network.learn(&replay_buffer)?;
        Ok(metrics)
    }
}
