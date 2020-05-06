use super::backpropagation;
use super::edges;
use super::expansion;
use super::nodes;
use super::selection;
use super::simulation;
use crate::algos;
use crate::rulesets;
use log;
use petgraph::graph;
use rand;
use rand::rngs;
use rand::seq::IteratorRandom;

pub struct MCTS<RuleSet: rulesets::Permutable> {
    ruleset: RuleSet,
    tree: graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    rng: rngs::ThreadRng,
    root: Option<graph::NodeIndex<u32>>,
}

impl<RuleSet: rulesets::Permutable> MCTS<RuleSet> {
    pub fn new(ruleset: RuleSet) -> MCTS<RuleSet> {
        MCTS {
            ruleset,
            tree: graph::Graph::new(),
            rng: rand::thread_rng(),
            root: None,
        }
    }

    pub fn set_state(&mut self, state: RuleSet::State) {
        let status = self.ruleset.status(&state);
        let index = self.tree.add_node(nodes::Node::new(state, status));
        self.root = Some(index);
    }

    pub fn iterate(&mut self) {
        log::debug!("###########################################");
        log::debug!("Starting new iteration");
        let node = match self.root {
            Some(node) => node,
            None => {
                log::debug!("No node selected, returning");
                return;
            }
        };
        log::debug!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        log::debug!("Selection phase");
        let mut selected = selection::select(&self.tree, node, false);
        log::debug!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        log::debug!("Expansion phase");
        let mut status = expansion::expand::<RuleSet>(&mut self.tree, &self.ruleset, selected);
        if let rulesets::Status::Ongoing = status {
            selected = match self.tree.neighbors(selected).choose(&mut self.rng) {
                Some(node) => node,
                None => selected,
            };
            let state = &self.tree.node_weight(selected).unwrap().state;
            log::debug!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            log::debug!("Simulation phase, node {:?}", selected);
            status = simulation::simulate::<RuleSet>(&self.ruleset, state, &mut self.rng);
        }
        log::debug!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        log::debug!("Backpropagation phase");
        backpropagation::backpropagate(&mut self.tree, selected, &status);
        log::debug!("Ending iteration");
    }

    pub fn play_scores(&self) -> Option<Vec<algos::PlyConsideration<RuleSet::Ply>>> {
        let parent = match self.root {
            Some(node) => node,
            None => {
                return None;
            }
        };
        let mut scores = self
            .tree
            .neighbors(parent)
            .map(|node_index| {
                let node_weight = self.tree.node_weight(node_index).unwrap();
                let edge = self.tree.find_edge(parent, node_index).unwrap();
                let edge_weight = self.tree.edge_weight(edge).unwrap();
                let follow_up = self.best_play(node_index);
                algos::PlyConsideration {
                    ply: edge_weight.ply,
                    score: node_weight.score(),
                    win_rate: node_weight.win_rate(),
                    draw_rate: node_weight.draw_rate(),
                    follow_up,
                }
            })
            .collect::<Vec<_>>();
        scores.sort_by(|consideration_a, consideration_b| {
            consideration_a
                .score
                .partial_cmp(&consideration_b.score)
                .unwrap()
                .reverse()
        });
        Some(scores)
    }

    fn best_play(&self, mut current_node: graph::NodeIndex<u32>) -> Vec<RuleSet::Ply> {
        let mut result = Vec::new();
        loop {
            let neighbours = self.tree.neighbors(current_node).map(|node_index| {
                let node_weight = self.tree.node_weight(node_index).unwrap();
                let edge = self.tree.find_edge(current_node, node_index).unwrap();
                let edge_weight = self.tree.edge_weight(edge).unwrap();
                (node_weight.score(), node_index, edge_weight.ply)
            });
            let best_neighbour = {
                neighbours.max_by(|(score_a, _, _), (score_b, _, _)| {
                    score_a.partial_cmp(score_b).unwrap()
                })
            };
            current_node = match best_neighbour {
                Some((_, node_index, ply)) => {
                    result.push(ply);
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
    use crate::rulesets::connectn;
    use crate::rulesets::RuleSetTrait;

    #[test]
    fn test_simulate() {
        let ruleset = connectn::TicTacToe::new();
        let state = ruleset.initial_state();
        let mut algo = MCTS::new(ruleset);
        algo.set_state(state);
        algo.iterate();
    }
}
