use super::backpropagation;
use super::edges;
use super::expansion;
use super::nodes;
use super::selection;
use super::simulation;
use crate::rulesets;
use petgraph::stable_graph;
use rand;
use rand::rngs;
use rand::seq::IteratorRandom;
use std::rc;

pub struct MCTS<RuleSet: rulesets::BaseRuleSet> {
    ruleset: RuleSet,
    tree: stable_graph::StableGraph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    rng: rngs::ThreadRng,
    root: Option<stable_graph::NodeIndex<u32>>,
}

impl<RuleSet: rulesets::BaseRuleSet> MCTS<RuleSet> {
    pub fn new(ruleset: RuleSet) -> MCTS<RuleSet> {
        MCTS {
            ruleset,
            tree: stable_graph::StableGraph::new(),
            rng: rand::thread_rng(),
            root: None,
        }
    }

    pub fn set_state(&mut self, state: rc::Rc<RuleSet::State>) {
        let index = self.tree.add_node(nodes::Node::new(state));
        self.root = Some(index);
    }

    pub fn iterate(&mut self, player: rulesets::Player) {
        let node = match self.root {
            Some(node) => node,
            None => {
                return;
            }
        };
        let selected = selection::select(&self.tree, node);
        expansion::expand::<RuleSet>(&mut self.tree, &self.ruleset, selected);
        let to_simulate = match self.tree.neighbors(selected).choose(&mut self.rng) {
            Some(node) => node,
            None => selected,
        };
        let state = self.tree.node_weight(to_simulate).unwrap().state.clone();
        let status = simulation::simulate::<RuleSet>(&self.ruleset, state, &mut self.rng);
        let player_status = status.player_pov(&player);
        backpropagation::backpropagate(&mut self.tree, to_simulate, &player_status);
    }

    pub fn play_scores(&self) {
        let parent = match self.root {
            Some(node) => node,
            None => {
                return;
            }
        };
        let mut scores = self
            .tree
            .neighbors(parent)
            .map(|node_index| {
                let node_weight = self.tree.node_weight(node_index).unwrap();
                let edge = self.tree.find_edge(parent, node_index).unwrap();
                let edge_weight = self.tree.edge_weight(edge).unwrap();
                (node_weight.score(), node_index, edge_weight.ply)
            })
            .collect::<Vec<_>>();
        scores.sort_by(|(score_a, _, _), (score_b, _, _)| score_a.partial_cmp(score_b).unwrap());
        for (score, node_index, ply) in scores {
            let scenario = self.best_play(node_index, true);
            println!("{:?}: {:?} => {:?}", ply, score, scenario);
        }
    }

    fn best_play(
        &self,
        mut current_node: stable_graph::NodeIndex<u32>,
        mut reverse: bool,
    ) -> Vec<RuleSet::Ply> {
        let mut result = Vec::new();
        loop {
            let neighbours = self.tree.neighbors(current_node).map(|node_index| {
                let node_weight = self.tree.node_weight(node_index).unwrap();
                let edge = self.tree.find_edge(current_node, node_index).unwrap();
                let edge_weight = self.tree.edge_weight(edge).unwrap();
                (node_weight.score(), node_index, edge_weight.ply)
            });
            let best_neighbour = if reverse {
                neighbours.min_by(|(score_a, _, _), (score_b, _, _)| {
                    score_a.partial_cmp(score_b).unwrap()
                })
            } else {
                neighbours.max_by(|(score_a, _, _), (score_b, _, _)| {
                    score_a.partial_cmp(score_b).unwrap()
                })
            };
            current_node = match best_neighbour {
                Some((_, node_index, ply)) => {
                    result.push(ply);
                    reverse = !reverse;
                    node_index
                }
                None => break,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rulesets::ninarow;
    use crate::rulesets::BaseRuleSet;

    #[test]
    fn test_simulate() {
        let ruleset = ninarow::TicTacToe::new();
        let state = rc::Rc::new(ruleset.initial_state());
        let mut algo = MCTS::new(ruleset);
        algo.set_state(state);
        algo.iterate(0);
    }
}
