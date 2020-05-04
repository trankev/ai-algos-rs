use super::items;
use crate::rulesets;
use petgraph::graph;

pub struct Response<RuleSet: rulesets::RuleSetTrait> {
    pub node_index: graph::NodeIndex<u32>,
    pub successors: Vec<items::PlyAndState<RuleSet>>,
}
