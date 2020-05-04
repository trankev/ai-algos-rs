use super::edges;
use super::nodes;
use crate::rulesets;
use crate::rulesets::PermutationIteratorTrait;
use crate::rulesets::PlyIteratorTrait;
use petgraph::graph;
use std::collections;
use std::rc;

pub fn expand<RuleSet: rulesets::Permutable>(
    tree: &mut graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    ruleset: &RuleSet,
    node: graph::NodeIndex<u32>,
) {
    let weight = tree.node_weight(node).unwrap();
    if weight.visits == 0.0 {
        return;
    }
    let current_state = weight.state.clone();

    let mut iterator = Expander::new(current_state);

    while let Some(PlyAndState { ply, state }) = iterator.iterate(ruleset) {
        let child_index = tree.add_node(nodes::Node::new(state));
        tree.add_edge(node, child_index, edges::Edge::new(ply));
    }
}

pub struct PlyAndState<RuleSet: rulesets::RuleSetTrait> {
    ply: RuleSet::Ply,
    state: rc::Rc<RuleSet::State>,
}

pub struct Expander<RuleSet: rulesets::Permutable> {
    ply_iterator: RuleSet::PlyIterator,
    current_state: rc::Rc<RuleSet::State>,
    seen: collections::HashSet<RuleSet::State>,
}

impl<RuleSet: rulesets::Permutable> Expander<RuleSet> {
    pub fn new(current_state: rc::Rc<RuleSet::State>) -> Expander<RuleSet> {
        let ply_iterator = RuleSet::PlyIterator::new(current_state.clone());
        Expander {
            current_state,
            ply_iterator,
            seen: collections::HashSet::new(),
        }
    }

    pub fn iterate(&mut self, ruleset: &RuleSet) -> Option<PlyAndState<RuleSet>> {
        for ply in self.ply_iterator.by_ref() {
            let resulting_state = rc::Rc::new(ruleset.play(&self.current_state, &ply).unwrap());
            let permutations = RuleSet::PermutationIterator::new(&ruleset);
            let witness_state = permutations
                .map(|permutation| ruleset.swap_state(&resulting_state, &permutation))
                .min()
                .unwrap();
            if self.seen.contains(&witness_state) {
                continue;
            }
            self.seen.insert(witness_state);
            return Some(PlyAndState {
                ply,
                state: resulting_state,
            });
        }
        None
    }
}
