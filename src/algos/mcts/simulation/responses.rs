use crate::interface;
use petgraph::graph;

pub struct Response {
    pub node_index: graph::NodeIndex<u32>,
    pub status: interface::Status,
}
