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
}

impl<RuleSet: rulesets::BaseRuleSet> MCTS<RuleSet> {
    pub fn new(ruleset: RuleSet) -> MCTS<RuleSet> {
        MCTS {
            ruleset,
            tree: stable_graph::StableGraph::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn compute<PlyIterator: rulesets::PlyIterator<RuleSet>>(
        &mut self,
        state: rc::Rc<RuleSet::State>,
        player: u8,
    ) {
        let index = self.tree.add_node(nodes::Node::new(state));
        self.iterate::<PlyIterator>(index, player);
    }

    fn iterate<PlyIterator: rulesets::PlyIterator<RuleSet>>(
        &mut self,
        node: stable_graph::NodeIndex<u32>,
        player: u8,
    ) {
        let selected = selection::select(&self.tree, node);
        expansion::expand::<RuleSet, PlyIterator>(&mut self.tree, &self.ruleset, selected);
        let to_simulate = match self.tree.neighbors(selected).choose(&mut self.rng) {
            Some(node) => node,
            None => selected,
        };
        let state = self.tree.node_weight(to_simulate).unwrap().state.clone();
        simulation::simulate::<RuleSet, PlyIterator>(&self.ruleset, state, &mut self.rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rulesets::ninarow;
    use crate::rulesets::ninarow::ply_iterators;
    use crate::rulesets::BaseRuleSet;

    #[test]
    fn test_simulate() {
        let ruleset = ninarow::TicTacToe::new();
        let state = rc::Rc::new(ruleset.initial_state());
        let mut algo = MCTS::new(ruleset);
        algo.compute::<ply_iterators::TicTacToePlyIterator>(state, 0);
    }
}
