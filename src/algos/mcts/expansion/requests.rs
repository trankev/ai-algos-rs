use crate::rulesets;
use petgraph::graph;

pub struct Request<RuleSet: rulesets::RuleSetTrait> {
    pub node_index: graph::NodeIndex<u32>,
    pub state: RuleSet::State,
}
