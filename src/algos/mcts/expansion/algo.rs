use super::items;
use super::iterator;
use crate::rulesets;
use crate::rulesets::StateTrait;
use log;

use super::super::edges;
use super::super::nodes;

use petgraph::graph;

pub fn expand<RuleSet: rulesets::Permutable>(
    tree: &mut graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    ruleset: &RuleSet,
    node: graph::NodeIndex<u32>,
) -> rulesets::Status {
    log::debug!("Expanding node {:?}", node);
    let weight = tree.node_weight_mut(node).unwrap();
    if weight.is_terminal() {
        log::debug!("Node is terminal, returning");
        return weight.global_status();
    }
    if !weight.is_visited() {
        log::debug!("Not yet visited, not expanding it yet");
        return rulesets::Status::Ongoing;
    }
    let mut iterator = iterator::Expander::new(weight.state.clone());

    while let Some(items::Play { ply, state, status }) = iterator.iterate(ruleset) {
        log::debug!(
            "Adding node as child of {:?} (ply: {:?}, status: {:?}, state: {:?})",
            node,
            ply,
            status,
            state.ascii_representation()
        );
        let child_index = tree.add_node(nodes::Node::new(state, status));
        log::debug!("Child node: {:?}", child_index);
        tree.add_edge(node, child_index, edges::Edge::new(ply));
    }
    rulesets::Status::Ongoing
}
