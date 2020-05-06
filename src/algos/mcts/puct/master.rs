use super::super::backpropagation;
use super::super::edges;
use super::super::expansion;
use super::super::nodes;
use super::super::selection;
use super::super::simulation;
use crate::rulesets;
use crossbeam::channel;
use petgraph::graph;
use rand;
use rand::rngs;
use rand::seq::IteratorRandom;
use std::error;

pub struct Master<RuleSet: rulesets::Permutable + 'static> {
    tree: graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    root: Option<graph::NodeIndex<u32>>,

    rng: rngs::ThreadRng,

    expansion_request_sender: channel::Sender<expansion::Request<RuleSet>>,
    expansion_response_receiver: channel::Receiver<expansion::Response<RuleSet>>,
    simulation_request_sender: channel::Sender<simulation::Request<RuleSet>>,
    simulation_response_receiver: channel::Receiver<simulation::Response>,
}

impl<RuleSet: rulesets::Permutable + 'static> Master<RuleSet> {
    pub fn new(
        expansion_request_sender: channel::Sender<expansion::Request<RuleSet>>,
        expansion_response_receiver: channel::Receiver<expansion::Response<RuleSet>>,
        simulation_request_sender: channel::Sender<simulation::Request<RuleSet>>,
        simulation_response_receiver: channel::Receiver<simulation::Response>,
    ) -> Master<RuleSet> {
        Master {
            tree: graph::Graph::new(),
            root: None,
            rng: rand::thread_rng(),
            expansion_request_sender,
            expansion_response_receiver,
            simulation_request_sender,
            simulation_response_receiver,
        }
    }

    pub fn set_state(&mut self, state: RuleSet::State, status: rulesets::Status) {
        let index = self.tree.add_node(nodes::Node::new(state, status));
        self.root = Some(index);
    }

    pub fn iterate(&mut self) -> Result<(), Box<dyn error::Error>> {
        let node = match self.root {
            Some(node) => node,
            None => {
                return Ok(());
            }
        };
        let selected = selection::select(&self.tree, node, false);
        self.expand(selected)?;
        let (to_simulate, status) = self.simulate(selected)?;
        backpropagation::backpropagate(&mut self.tree, to_simulate, &status);
        Ok(())
    }

    fn expand(&mut self, node_index: graph::NodeIndex<u32>) -> Result<(), Box<dyn error::Error>> {
        let weight = self.tree.node_weight(node_index).unwrap();
        if weight.is_terminal() || !weight.is_visited() {
            return Ok(());
        }
        let request = expansion::Request {
            node_index,
            state: weight.state.clone(),
        };
        self.expansion_request_sender.send(request).unwrap();
        let response = self.expansion_response_receiver.recv().unwrap();
        for successor in response.successors {
            let node_weight = nodes::Node::new(successor.state, successor.status);
            let child_index = self.tree.add_node(node_weight);
            let edge_weight = edges::Edge::new(successor.ply);
            self.tree
                .add_edge(response.node_index, child_index, edge_weight);
        }
        Ok(())
    }

    fn simulate(
        &mut self,
        node_index: graph::NodeIndex<u32>,
    ) -> Result<(graph::NodeIndex<u32>, rulesets::Status), Box<dyn error::Error>> {
        let to_simulate = match self.tree.neighbors(node_index).choose(&mut self.rng) {
            Some(node) => node,
            None => node_index,
        };
        let state = self.tree.node_weight(to_simulate).unwrap().state.clone();
        let request = simulation::Request { node_index, state };
        self.simulation_request_sender.send(request)?;
        let response = self.simulation_response_receiver.recv()?;
        Ok((response.node_index, response.status))
    }
}
