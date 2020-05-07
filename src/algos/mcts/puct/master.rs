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

    pub fn first_iteration(&mut self) -> Result<(), Box<dyn error::Error>> {
        let node = match self.root {
            Some(node) => node,
            None => {
                return Ok(());
            }
        };

        let selected = selection::select(&self.tree, node, false);

        let mut wait_for_expansion = false;
        match expansion::ponder_expansion::<RuleSet>(&self.tree, selected, false) {
            expansion::ExpansionStatus::RequiresExpansion(state) => {
                let request = expansion::Request::ExpansionRequest {
                    node_index: selected,
                    state,
                };
                self.expansion_request_sender.send(request)?;
                wait_for_expansion = true;
            }
            expansion::ExpansionStatus::NotVisited => unreachable!(),
            expansion::ExpansionStatus::Terminal(_) => (),
        };

        let (to_simulate, state) =
            simulation::fetch_random_child::<RuleSet>(&self.tree, selected, &mut self.rng);
        let request = simulation::Request::SimulationRequest {
            node_index: to_simulate,
            state,
        };
        self.simulation_request_sender.send(request)?;

        if wait_for_expansion {
            let response = self.expansion_response_receiver.recv()?;
            for successor in response.successors {
                expansion::save_expansion(&mut self.tree, selected, successor);
            }
        }

        let response = self.simulation_response_receiver.recv()?;
        backpropagation::backpropagate(&mut self.tree, response.node_index, &response.status);

        Ok(())
    }
}
