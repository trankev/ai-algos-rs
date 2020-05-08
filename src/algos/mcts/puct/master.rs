use super::super::analysis;
use super::super::backpropagation;
use super::super::edges;
use super::super::expansion;
use super::super::nodes;
use super::super::selection;
use super::super::simulation;
use super::requests;
use super::responses;
use crate::algos;
use crate::rulesets;
use crossbeam::channel;
use petgraph::graph;
use rand;
use rand::rngs;
use std::error;

pub struct Master<RuleSet: rulesets::Permutable + 'static> {
    tree: graph::Graph<nodes::Node<RuleSet::State>, edges::Edge<RuleSet::Ply>>,
    root: Option<graph::NodeIndex<u32>>,
    ruleset: RuleSet,

    rng: rngs::ThreadRng,

    master_request_receiver: channel::Receiver<requests::Request<RuleSet>>,
    master_response_sender: channel::Sender<responses::Response<RuleSet>>,
    expansion_request_sender: channel::Sender<expansion::Request<RuleSet>>,
    expansion_response_receiver: channel::Receiver<expansion::Response<RuleSet>>,
    simulation_request_sender: channel::Sender<simulation::Request<RuleSet>>,
    simulation_response_receiver: channel::Receiver<simulation::Response>,
}

impl<RuleSet: rulesets::Permutable + 'static> Master<RuleSet> {
    pub fn new(
        ruleset: RuleSet,
        master_request_receiver: channel::Receiver<requests::Request<RuleSet>>,
        master_response_sender: channel::Sender<responses::Response<RuleSet>>,
        expansion_request_sender: channel::Sender<expansion::Request<RuleSet>>,
        expansion_response_receiver: channel::Receiver<expansion::Response<RuleSet>>,
        simulation_request_sender: channel::Sender<simulation::Request<RuleSet>>,
        simulation_response_receiver: channel::Receiver<simulation::Response>,
    ) -> Master<RuleSet> {
        Master {
            tree: graph::Graph::new(),
            root: None,
            ruleset,
            rng: rand::thread_rng(),
            master_request_receiver,
            master_response_sender,
            expansion_request_sender,
            expansion_response_receiver,
            simulation_request_sender,
            simulation_response_receiver,
        }
    }

    fn set_state(&mut self, state: RuleSet::State) {
        let status = self.ruleset.status(&state);
        let index = self.tree.add_node(nodes::Node::new(state, status));
        self.root = Some(index);
    }

    fn first_iteration(&mut self) -> Result<(), Box<dyn error::Error>> {
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

    fn iterate_sequential(&mut self) -> Result<(), Box<dyn error::Error>> {
        let node = match self.root {
            Some(node) => node,
            None => {
                return Ok(());
            }
        };

        let selected = selection::select(&self.tree, node, false);

        match expansion::ponder_expansion::<RuleSet>(&self.tree, selected, false) {
            expansion::ExpansionStatus::RequiresExpansion(state) => {
                let request = expansion::Request::ExpansionRequest {
                    node_index: selected,
                    state,
                };
                self.expansion_request_sender.send(request)?;
                let response = self.expansion_response_receiver.recv()?;
                for successor in response.successors {
                    expansion::save_expansion(&mut self.tree, selected, successor);
                }
            }
            expansion::ExpansionStatus::NotVisited => (),
            expansion::ExpansionStatus::Terminal(status) => {
                backpropagation::backpropagate(&mut self.tree, selected, &status);
                return Ok(());
            }
        }

        let (to_simulate, state) =
            simulation::fetch_random_child::<RuleSet>(&self.tree, selected, &mut self.rng);
        let request = simulation::Request::SimulationRequest {
            node_index: to_simulate,
            state,
        };
        self.simulation_request_sender.send(request)?;
        let response = self.simulation_response_receiver.recv()?;

        backpropagation::backpropagate(&mut self.tree, response.node_index, &response.status);

        Ok(())
    }

    fn play_scores(&self) -> Option<Vec<algos::PlyConsideration<RuleSet::Ply>>> {
        let parent = match self.root {
            Some(node) => node,
            None => {
                return None;
            }
        };
        Some(analysis::play_scores::<RuleSet>(&self.tree, parent))
    }

    pub fn run(&mut self) -> Result<(), Box<dyn error::Error>> {
        loop {
            match self.master_request_receiver.recv()? {
                requests::Request::SetState(state) => {
                    self.set_state(state);
                    self.first_iteration()?;
                }
                requests::Request::Iterate { count } => {
                    for _ in 0..count {
                        self.iterate_sequential()?;
                    }
                }
                requests::Request::ListConsiderations => {
                    let result = self.play_scores().unwrap();
                    let response = responses::Response::PlyConsiderations(result);
                    self.master_response_sender.send(response)?;
                }
                requests::Request::Stop => break,
            }
        }
        Ok(())
    }
}
