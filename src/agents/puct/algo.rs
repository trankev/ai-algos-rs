use super::nodes;
use crate::interface::ai;
use crate::interface::rulesets;
use std::collections;
use std::error;
use std::hash;

pub struct PUCT<'a, RuleSet, Policy>
where
    RuleSet: rulesets::RuleSetTrait,
    Policy: ai::Policy<RuleSet>,
{
    ruleset: &'a RuleSet,
    inner_policy: &'a mut Policy,
    state_indices: collections::HashMap<RuleSet::State, usize>,
    nodes: Vec<nodes::Node<RuleSet>>,
    cpuct: f32,
    simulation_count: usize,
}

impl<'a, RuleSet, Policy> PUCT<'a, RuleSet, Policy>
where
    RuleSet: rulesets::TurnByTurn + rulesets::Deterministic,
    RuleSet::State: Eq + hash::Hash,
    Policy: ai::Policy<RuleSet>,
{
    pub fn new(
        ruleset: &'a RuleSet,
        inner_policy: &'a mut Policy,
        cpuct: f32,
        simulation_count: usize,
    ) -> PUCT<'a, RuleSet, Policy> {
        PUCT {
            ruleset,
            inner_policy,
            state_indices: collections::HashMap::new(),
            nodes: Vec::new(),
            cpuct,
            simulation_count,
        }
    }

    fn search(&mut self, state: &RuleSet::State) -> Result<f32, Box<dyn error::Error>> {
        let (node_index, child_index, resulting_state) = match self.state_indices.get(state) {
            Some(index) => {
                let node = &self.nodes[*index];
                match &node.status {
                    nodes::NodeStatus::Terminal => return Ok(node.value),
                    nodes::NodeStatus::Ongoing { children } => {
                        let (child_index, child) = children
                            .iter()
                            .enumerate()
                            .max_by(|(_, child1), (_, child2)| {
                                let ucb1 = child1.ucb(node.visits, self.cpuct);
                                let ucb2 = child2.ucb(node.visits, self.cpuct);
                                ucb1.partial_cmp(&ucb2).unwrap()
                            })
                            .unwrap();
                        let resulting_state = self.ruleset.play(&state, &child.ply).unwrap();
                        (*index, child_index, resulting_state)
                    }
                }
            }
            None => {
                let status = self.ruleset.status(state);
                let (node_state, value) = match status {
                    rulesets::Status::Ongoing => {
                        let prediction = self.inner_policy.predict(state)?;
                        let children = prediction
                            .probabilities
                            .iter()
                            .map(|(ply, probability)| nodes::Child::new(*ply, *probability))
                            .collect();
                        let node_state = nodes::NodeStatus::Ongoing { children };
                        (node_state, prediction.value)
                    }
                    _ => {
                        let node_state = nodes::NodeStatus::Terminal;
                        let qvalue = match status.player_pov(&self.ruleset.current_player(&state)) {
                            rulesets::PlayerStatus::Win => 1.0,
                            rulesets::PlayerStatus::Draw => 0.0,
                            rulesets::PlayerStatus::Loss => -1.0,
                            rulesets::PlayerStatus::Ongoing => unreachable!(), // first match must have returned
                        };
                        (node_state, qvalue)
                    }
                };
                let index = self.nodes.len();
                let node = nodes::Node {
                    status: node_state,
                    value,
                    visits: 1e-8,
                };
                self.nodes.push(node);
                self.state_indices.insert(state.clone(), index);
                return Ok(value);
            }
        };
        let value = -self.search(&resulting_state)?;
        match &mut self.nodes[node_index].status {
            nodes::NodeStatus::Terminal => unreachable!(),
            nodes::NodeStatus::Ongoing { children } => {
                let child = &mut children[child_index];
                child.update(value);
            }
        }
        Ok(value)
    }
}

impl<'a, RuleSet, Policy> ai::Policy<RuleSet> for PUCT<'a, RuleSet, Policy>
where
    RuleSet: rulesets::TurnByTurn + rulesets::Deterministic,
    RuleSet::State: Eq + hash::Hash,
    Policy: ai::Policy<RuleSet>,
{
    fn predict(
        &mut self,
        state: &RuleSet::State,
    ) -> Result<ai::Prediction<RuleSet>, Box<dyn error::Error>> {
        for _ in 0..self.simulation_count {
            self.search(state)?;
        }
        let state_index = self.state_indices.get(state).unwrap(); // search method created it
        let node = &self.nodes[*state_index];
        let prediction = match &node.status {
            nodes::NodeStatus::Terminal => ai::Prediction {
                value: node.value,
                probabilities: Vec::new(),
            },
            nodes::NodeStatus::Ongoing { children } => {
                let total_visits = children.iter().fold(0.0, |acc, child| acc + child.visits);
                let probabilities = children
                    .iter()
                    .map(|child| (child.ply, child.visits / total_visits))
                    .collect();
                let value = 0.0;
                ai::Prediction {
                    value,
                    probabilities,
                }
            }
        };
        Ok(prediction)
    }
}
