use super::nodes;
use crate::rulesets;
use petgraph;
use petgraph::graph;

pub fn backpropagate<State: rulesets::StateTrait, Edge>(
    tree: &mut graph::Graph<nodes::Node<State>, Edge>,
    node: graph::NodeIndex<u32>,
    status: &rulesets::Status,
) {
    let mut neighbours = tree
        .neighbors_directed(node, petgraph::Direction::Incoming)
        .detach();
    while let Some((_, parent)) = neighbours.next(tree) {
        backpropagate(tree, parent, status);
    }
    update_tallies(tree, node, status);
}

fn update_tallies<State: rulesets::StateTrait, Edge>(
    tree: &mut graph::Graph<nodes::Node<State>, Edge>,
    node: graph::NodeIndex<u32>,
    status: &rulesets::Status,
) {
    let weight = tree.node_weight_mut(node).unwrap();
    weight.add_visit();
    weight.backpropagate(status);
}
