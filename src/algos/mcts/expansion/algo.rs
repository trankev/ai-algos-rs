use super::items;
use super::iterator;
use crate::rulesets;
use crate::rulesets::StateTrait;

use super::super::edges;
use super::super::nodes;

use petgraph::graph;

pub fn expand<RuleSet: rulesets::Permutable>(
    tree: &mut graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    ruleset: &RuleSet,
    node: graph::NodeIndex<u32>,
) -> rulesets::Status {
    let weight = tree.node_weight_mut(node).unwrap();
    if weight.is_terminal() {
        return weight.global_status();
    }
    if !weight.is_visited() {
        let status = ruleset.status(&weight.state);
        match status {
            rulesets::Status::Ongoing => return rulesets::Status::Ongoing,
            _ => {
                weight.set_terminal(status);
                return status;
            }
        }
    }
    let mut iterator = iterator::Expander::new(weight.state.clone());

    while let Some(items::PlyAndState { ply, state }) = iterator.iterate(ruleset) {
        let child_index = tree.add_node(nodes::Node::new(state));
        tree.add_edge(node, child_index, edges::Edge::new(ply));
    }
    rulesets::Status::Ongoing
}
