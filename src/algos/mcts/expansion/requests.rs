use crate::rulesets;
use petgraph::graph;

pub enum Request<RuleSet: rulesets::RuleSetTrait> {
    ExpansionRequest {
        node_index: graph::NodeIndex<u32>,
        state: RuleSet::State,
    },
    Stop,
}
