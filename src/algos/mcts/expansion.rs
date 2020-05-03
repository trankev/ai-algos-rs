use super::edges;
use super::nodes;
use crate::rulesets;
use crate::rulesets::PlyIteratorTrait;
use petgraph::graph;
use std::rc;

pub fn expand<RuleSet: rulesets::RuleSetTrait>(
    tree: &mut graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    ruleset: &RuleSet,
    node: graph::NodeIndex<u32>,
) {
    let state = tree.node_weight(node).unwrap().state.clone();
    let available_plies = RuleSet::PlyIterator::new(state.clone());
    for ply in available_plies {
        let resulting_state = rc::Rc::new(ruleset.play(&state, &ply).unwrap());
        let child_index = tree.add_node(nodes::Node::new(resulting_state));
        tree.add_edge(node, child_index, edges::Edge::new(ply));
    }
}
