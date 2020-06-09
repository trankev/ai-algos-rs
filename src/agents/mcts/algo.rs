use super::analysis;
use super::backpropagation;
use super::edges;
use super::expansion;
use super::nodes;
use super::selection;
use super::simulation;
use crate::interface::ai;
use crate::interface::rulesets;
use crate::interface::rulesets::StateTrait;
use petgraph::graph;
use rand;
use rand::rngs;
use std::hash;

pub struct MCTS<RuleSet>
where
    RuleSet: rulesets::HasStatesWithSymmetries + rulesets::Deterministic + rulesets::TurnByTurn,
    RuleSet::Ply: Eq + Ord + hash::Hash,
    RuleSet::State: Eq,
{
    ruleset: RuleSet,
    tree: graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    rng: rngs::ThreadRng,
    root: Option<graph::NodeIndex<u32>>,
    pub expansion_count: usize,
    pub simulation_count: usize,
}

impl<RuleSet> MCTS<RuleSet>
where
    RuleSet: rulesets::HasStatesWithSymmetries + rulesets::Deterministic + rulesets::TurnByTurn,
    RuleSet::Ply: Eq + Ord + hash::Hash,
    RuleSet::State: Eq,
{
    pub fn new(ruleset: RuleSet) -> MCTS<RuleSet> {
        MCTS {
            ruleset,
            tree: graph::Graph::new(),
            rng: rand::thread_rng(),
            root: None,
            expansion_count: 0,
            simulation_count: 0,
        }
    }

    pub fn set_state(&mut self, state: RuleSet::State) {
        let status = self.ruleset.status(&state);
        let current_player = self.ruleset.current_player(&state);
        let index = self
            .tree
            .add_node(nodes::Node::new(state, status, current_player));
        self.root = Some(index);
    }

    pub fn iterate(&mut self) {
        let node = match self.root {
            Some(node) => node,
            None => {
                return;
            }
        };
        let mut selected = selection::select(&self.tree, node);
        let (mut status, expanded) =
            expansion::expand::<RuleSet>(&mut self.tree, &self.ruleset, selected);
        if expanded {
            self.expansion_count += 1;
        }
        if let rulesets::Status::Ongoing = status {
            self.simulation_count += 1;
            let (to_simulate, state) =
                simulation::fetch_random_child::<RuleSet>(&self.tree, selected, &mut self.rng);
            status = simulation::simulate::<RuleSet>(&self.ruleset, &state, &mut self.rng);
            selected = to_simulate;
        }
        backpropagation::backpropagate(&mut self.tree, selected, true, Some(&status));
    }

    pub fn play_scores(&self) -> Option<Vec<ai::PlyConsideration<RuleSet::Ply>>> {
        let parent = match self.root {
            Some(node) => node,
            None => {
                return None;
            }
        };
        Some(analysis::play_scores::<RuleSet>(&self.tree, parent))
    }

    pub fn walk_best(&mut self) {
        let parent = match self.root {
            Some(node) => node,
            None => {
                return;
            }
        };
        let (node_index, _) = self
            .tree
            .neighbors(parent)
            .map(|node_index| {
                let node_weight = self.tree.node_weight(node_index).unwrap();
                (node_index, node_weight.score())
            })
            .max_by(|(_, score_a), (_, score_b)| score_a.partial_cmp(&score_b).unwrap())
            .unwrap();
        self.root = Some(node_index);
        let node_weight = self.tree.node_weight(node_index).unwrap();
        println!("{}", node_weight.state.ascii_representation());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::rulesets::RuleSetTrait;
    use crate::rulesets::connectn;

    #[test]
    fn test_simulate() {
        let ruleset = connectn::TicTacToe::new();
        let state = ruleset.initial_state();
        let mut algo = MCTS::new(ruleset);
        algo.set_state(state);
        algo.iterate();
    }
}
