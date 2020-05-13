use super::items;
use super::iterator;
use crate::rulesets;

use super::super::edges;
use super::super::nodes;

use petgraph::graph;

pub enum ExpansionStatus<State> {
    RequiresExpansion(State),
    NotVisited,
    Terminal(rulesets::Status),
    PendingExpansion,
}

pub fn expand<RuleSet: rulesets::Permutable>(
    tree: &mut graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    ruleset: &RuleSet,
    node: graph::NodeIndex<u32>,
) -> (rulesets::Status, bool) {
    let state = match ponder_expansion::<RuleSet>(tree, node, true) {
        ExpansionStatus::RequiresExpansion(state) => state,
        ExpansionStatus::NotVisited => return (rulesets::Status::Ongoing, false),
        ExpansionStatus::Terminal(status) => return (status, false),
        ExpansionStatus::PendingExpansion => unreachable!(),
    };
    let mut iterator = iterator::Expander::new(state);

    while let Some(successor) = iterator.iterate(ruleset) {
        save_expansion(tree, node, successor);
    }
    (rulesets::Status::Ongoing, true)
}

pub fn ponder_expansion<RuleSet: rulesets::RuleSetTrait>(
    tree: &mut graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    node_index: graph::NodeIndex<u32>,
    check_for_visits: bool,
) -> ExpansionStatus<RuleSet::State> {
    let weight = tree.node_weight_mut(node_index).unwrap();
    if weight.expanding {
        return ExpansionStatus::PendingExpansion;
    }
    let status = weight.game_status();
    match status {
        rulesets::Status::Ongoing => (),
        _ => return ExpansionStatus::Terminal(status),
    }
    if check_for_visits && !weight.is_visited() {
        return ExpansionStatus::NotVisited;
    }
    weight.expanding = true;
    ExpansionStatus::RequiresExpansion(weight.state.clone())
}

pub fn save_expansion<RuleSet: rulesets::RuleSetTrait>(
    tree: &mut graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    node_index: graph::NodeIndex<u32>,
    successor: items::Play<RuleSet>,
) {
    let mut parent_weight = tree.node_weight_mut(node_index).unwrap();
    parent_weight.expanding = false;
    let node_weight = nodes::Node::new(successor.state, successor.status);
    let child_index = tree.add_node(node_weight);
    let edge_weight = edges::Edge::new(successor.ply);
    tree.add_edge(node_index, child_index, edge_weight);
}
