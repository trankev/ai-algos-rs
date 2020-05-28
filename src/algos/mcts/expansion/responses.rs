use super::items;
use crate::interface::rulesets;
use petgraph::graph;

pub struct Response<RuleSet: rulesets::RuleSetTrait> {
    pub node_index: graph::NodeIndex<u32>,
    pub successors: Vec<items::Play<RuleSet>>,
}
