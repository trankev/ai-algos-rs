use super::nodes;
use crate::rulesets;
use petgraph;
use petgraph::graph;

pub fn backpropagate<State, Edge>(
    tree: &mut graph::Graph<nodes::Node<State>, Edge>,
    node: graph::NodeIndex<u32>,
    status: &rulesets::PlayerStatus,
) {
    let mut neighbours = tree
        .neighbors_directed(node, petgraph::Direction::Incoming)
        .detach();
    while let Some((_, parent)) = neighbours.next(tree) {
        backpropagate(tree, parent, status);
    }
    update_tallies(tree, node, status);
}

fn update_tallies<State, Edge>(
    tree: &mut graph::Graph<nodes::Node<State>, Edge>,
    node: graph::NodeIndex<u32>,
    status: &rulesets::PlayerStatus,
) {
    let mut weight = tree.node_weight_mut(node).unwrap();
    weight.visits += 1.0;
    match status {
        rulesets::PlayerStatus::Win => weight.wins += 1.0,
        rulesets::PlayerStatus::Draw => weight.draws += 1.0,
        _ => (),
    }
}
