use super::nodes;
use crate::interface;
use petgraph;
use petgraph::graph;

pub fn backpropagate<State: interface::StateTrait + interface::TurnByTurnState, Edge>(
    tree: &mut graph::Graph<nodes::Node<State>, Edge>,
    node: graph::NodeIndex<u32>,
    update_visits: bool,
    status: Option<&interface::Status>,
) {
    let mut neighbours = tree
        .neighbors_directed(node, petgraph::Direction::Incoming)
        .detach();
    while let Some((_, parent)) = neighbours.next(tree) {
        backpropagate(tree, parent, update_visits, status);
    }
    update_tallies(tree, node, update_visits, status);
}

pub fn update_tallies<State: interface::StateTrait + interface::TurnByTurnState, Edge>(
    tree: &mut graph::Graph<nodes::Node<State>, Edge>,
    node: graph::NodeIndex<u32>,
    update_visits: bool,
    status: Option<&interface::Status>,
) {
    let weight = tree.node_weight_mut(node).unwrap();
    if update_visits {
        weight.add_visit();
    }
    if let Some(status) = status {
        weight.backpropagate(status);
    }
}
