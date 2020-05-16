use super::super::edges;
use super::super::nodes;
use crate::interface;
use crate::tools::ply_iterator;
use petgraph::graph;
use rand::rngs;
use rand::seq::IteratorRandom;

pub fn simulate<RuleSet: interface::RuleSetTrait>(
    ruleset: &RuleSet,
    state: &RuleSet::State,
    rng: &mut rngs::ThreadRng,
) -> interface::Status {
    let mut current_state = state;
    let mut state;
    loop {
        let status = ruleset.status(current_state);
        if let interface::Status::Ongoing = status {
            let available_plies = ply_iterator::PlyIterator::new(ruleset, current_state);
            let ply = available_plies.choose(rng).unwrap();
            state = ruleset.play(&current_state, &ply).unwrap();
            current_state = &state;
        } else {
            return status;
        }
    }
}

pub fn fetch_random_child<RuleSet: interface::RuleSetTrait>(
    tree: &graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    node_index: graph::NodeIndex<u32>,
    rng: &mut rngs::ThreadRng,
) -> (graph::NodeIndex<u32>, RuleSet::State) {
    let to_simulate = match tree.neighbors(node_index).choose(rng) {
        Some(node) => node,
        None => node_index,
    };
    let state = tree.node_weight(to_simulate).unwrap().state.clone();
    (to_simulate, state)
}
