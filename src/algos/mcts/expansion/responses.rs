use super::items;
use crate::interface;
use petgraph::graph;

pub struct Response<RuleSet: interface::RuleSetTrait> {
    pub node_index: graph::NodeIndex<u32>,
    pub successors: Vec<items::Play<RuleSet>>,
}
