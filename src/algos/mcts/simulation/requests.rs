use crate::interface;
use petgraph::graph;

pub enum Request<RuleSet: interface::RuleSetTrait> {
    SimulationRequest {
        node_index: graph::NodeIndex<u32>,
        state: RuleSet::State,
    },
    Stop,
}
