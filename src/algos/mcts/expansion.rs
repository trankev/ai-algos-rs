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
    let state = weight.state.clone();
    let mut seen = collections::HashSet::new();
    let available_plies = RuleSet::PlyIterator::new(state.clone());
    for ply in available_plies {
        let resulting_state = rc::Rc::new(ruleset.play(&state, &ply).unwrap());
        let permutations = RuleSet::PermutationIterator::new(&ruleset);
        let witness_state = permutations
            .map(|permutation| ruleset.swap_state(&resulting_state, &permutation))
            .min()
            .unwrap();
        if seen.contains(&witness_state) {
            continue;
        }
        let child_index = tree.add_node(nodes::Node::new(resulting_state));
        tree.add_edge(node, child_index, edges::Edge::new(ply));
        seen.insert(witness_state);
    }
}
