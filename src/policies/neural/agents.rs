use super::implementations;
use super::metrics;
use super::networks;
use super::ply_encoding;
use super::samples;
use crate::interface::ai;
use crate::interface::rulesets;
use crate::tools::plies;
use rand::rngs;
use rand::Rng;
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
    rng: rngs::ThreadRng,
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
        let rng = rand::thread_rng();
        let agent = Agent {
            ruleset,
            network,
            implementation,
            rng,
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

impl<'a, RuleSet, Implementation> ai::Teachable<RuleSet> for Agent<'a, RuleSet, Implementation>
where
    RuleSet: rulesets::HasStatesWithSymmetries + rulesets::TurnByTurn,
    Implementation: implementations::Implementation<RuleSet>,
    RuleSet::Ply: Ord + hash::Hash,
    RuleSet::State: Eq,
{
    type Metrics = metrics::Metrics;

    fn learn(
        &mut self,
        logs: &[ai::PolicyLog<RuleSet>],
    ) -> Result<metrics::Metrics, Box<dyn error::Error>> {
        let batch_size = 64;
        let epochs = 10;
        let state_count: usize = logs.iter().map(|log| log.history.len()).sum();
        let bucket_count = state_count / batch_size + 1;
        let mut buckets = Vec::new();
        for _ in 0..bucket_count {
            buckets.push(samples::TrainSample::new());
        }
        for log in logs {
            for turn in &log.history {
                let encoded_state = Implementation::encode_state(&turn.state);
                let predictions =
                    ply_encoding::encode::<RuleSet, Implementation>(&turn.prediction.probabilities);
                let reward = match log
                    .status
                    .player_pov(self.ruleset.current_player(&turn.state))
                {
                    rulesets::PlayerStatus::Win => 1.0,
                    rulesets::PlayerStatus::Draw => 0.0,
                    rulesets::PlayerStatus::Loss => -1.0,
                    rulesets::PlayerStatus::Ongoing => unreachable!(),
                };
                let bucket_index = {
                    let mut bucket_index = 0;
                    for _ in 0..10 {
                        bucket_index = self.rng.gen_range(0, bucket_count);
                        if buckets[bucket_index].size < batch_size as u64 {
                            break;
                        }
                    }
                    bucket_index
                };
                buckets[bucket_index].add(&encoded_state, reward, &predictions);
            }
        }
        let mut totals = metrics::Metrics {
            policy_loss: 0.0,
            value_loss: 0.0,
        };
        for _ in 0..epochs {
            for bucket in &buckets {
                let bucket_metrics = self.network.train(&bucket)?;
                totals.policy_loss += bucket_metrics.policy_loss;
                totals.value_loss += bucket_metrics.value_loss;
            }
        }
        let iterations = (epochs * bucket_count) as f32;
        totals.policy_loss /= iterations;
        totals.value_loss /= iterations;
        Ok(totals)
    }
}

impl<'a, RuleSet, Implementation> ai::WithMemory for Agent<'a, RuleSet, Implementation>
where
    RuleSet: rulesets::HasStatesWithSymmetries + rulesets::TurnByTurn,
    Implementation: implementations::Implementation<RuleSet>,
    RuleSet::Ply: Ord + hash::Hash,
    RuleSet::State: Eq,
{
    fn save<P: AsRef<path::Path>>(&self, project_folder: P) -> Result<(), Box<dyn error::Error>> {
        self.network.save(project_folder)
    }

    fn load<P: AsRef<path::Path>>(
        &mut self,
        project_folder: P,
    ) -> Result<(), Box<dyn error::Error>> {
        self.network.load(project_folder)
    }
}
