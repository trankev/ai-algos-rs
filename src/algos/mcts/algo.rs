use super::analysis;
use super::backpropagation;
use super::edges;
use super::expansion;
use super::nodes;
use super::selection;
use super::simulation;
use crate::algos;
use crate::rulesets;
use crate::rulesets::StateTrait;
use log;
use petgraph::graph;
use rand;
use rand::rngs;

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
            let (to_simulate, state) =
                simulation::fetch_random_child::<RuleSet>(&self.tree, selected, &mut self.rng);
            log::debug!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            log::debug!("Simulation phase, node {:?}", selected);
            status = simulation::simulate::<RuleSet>(&self.ruleset, &state, &mut self.rng);
            selected = to_simulate;
        }
        log::debug!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        log::debug!("Backpropagation phase");
        backpropagation::backpropagate(&mut self.tree, selected, true, Some(&status));
        log::debug!("Ending iteration");
    }

    pub fn play_scores(&self) -> Option<Vec<algos::PlyConsideration<RuleSet::Ply>>> {
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
