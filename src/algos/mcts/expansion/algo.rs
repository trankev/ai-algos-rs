use super::items;
use super::iterator;
use crate::rulesets;

use super::super::edges;
use super::super::nodes;

use petgraph::graph;

pub fn expand<RuleSet: rulesets::Permutable>(
    tree: &mut graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    ruleset: &RuleSet,
    node: graph::NodeIndex<u32>,
) {
    let weight = tree.node_weight(node).unwrap();
    if weight.visits == 0.0 {
        return;
    }
    let mut iterator = iterator::Expander::new(weight.state.clone());

    while let Some(items::PlyAndState { ply, state }) = iterator.iterate(ruleset) {
        let child_index = tree.add_node(nodes::Node::new(state));
        tree.add_edge(node, child_index, edges::Edge::new(ply));
    }
}
