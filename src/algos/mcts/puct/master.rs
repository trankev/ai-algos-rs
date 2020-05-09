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

enum SelectionResult {
    Expansion,
    Simulation,
    Nothing,
}

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

        let selected = selection::select(&self.tree, node, true);

        let mut wait_for_expansion = false;
        match expansion::ponder_expansion::<RuleSet>(&mut self.tree, selected, false) {
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
            expansion::ExpansionStatus::PendingExpansion => unreachable!(),
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
        backpropagation::backpropagate(
            &mut self.tree,
            response.node_index,
            true,
            Some(&response.status),
        );

        Ok(())
    }

    fn iterate_concurrent(
        &mut self,
        iteration_count: usize,
        expansion_workers: usize,
        simulation_workers: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        let node = match self.root {
            Some(node) => node,
            None => {
                return Ok(());
            }
        };
        let expansion_threshold = expansion_workers as isize;
        let simulation_threshold = simulation_workers as isize;
        let mut expansion_jobs: isize = 0;
        let mut simulation_jobs: isize = 0;
        for _ in 0..iteration_count {
            if simulation_jobs < 100 {
                match self.make_selection(node)? {
                    SelectionResult::Expansion => {
                        expansion_jobs += 1;
                    }
                    SelectionResult::Simulation => {
                        simulation_jobs += 1;
                    }
                    SelectionResult::Nothing => (),
                }
                if expansion_jobs < expansion_threshold {
                    continue;
                }
            }

            let jobs = self.wait_for_expansion()?;
            expansion_jobs -= jobs;
            simulation_jobs += jobs;
            if simulation_jobs < simulation_threshold {
                continue;
            }
            simulation_jobs -= self.wait_for_simulation()?;
        }
        println!(
            "Cleaning {} expansion_jobs and {} simulation_jobs",
            expansion_jobs, simulation_jobs
        );
        while expansion_jobs > 0 {
            let jobs = self.wait_for_expansion()?;
            expansion_jobs -= jobs;
            simulation_jobs += jobs;
        }
        while simulation_jobs > 0 {
            simulation_jobs -= self.wait_for_simulation()?;
        }
        Ok(())
    }

    fn make_selection(
        &mut self,
        node: graph::NodeIndex<u32>,
    ) -> Result<SelectionResult, Box<dyn error::Error>> {
        let selected = selection::select(&self.tree, node, false);
        match expansion::ponder_expansion::<RuleSet>(&mut self.tree, selected, true) {
            expansion::ExpansionStatus::RequiresExpansion(state) => {
                let request = expansion::Request::ExpansionRequest {
                    node_index: selected,
                    state,
                };
                self.expansion_request_sender.send(request)?;
                backpropagation::backpropagate(&mut self.tree, selected, true, None);
                Ok(SelectionResult::Expansion)
            }
            expansion::ExpansionStatus::NotVisited => {
                let (to_simulate, state) =
                    simulation::fetch_random_child::<RuleSet>(&self.tree, selected, &mut self.rng);
                let request = simulation::Request::SimulationRequest {
                    node_index: to_simulate,
                    state,
                };
                self.simulation_request_sender.send(request)?;
                backpropagation::backpropagate(&mut self.tree, selected, true, None);
                Ok(SelectionResult::Simulation)
            }
            expansion::ExpansionStatus::Terminal(status) => {
                backpropagation::backpropagate(&mut self.tree, selected, true, Some(&status));
                Ok(SelectionResult::Nothing)
            }
            expansion::ExpansionStatus::PendingExpansion => Ok(SelectionResult::Nothing),
        }
    }

    fn handle_expansion(
        &mut self,
        node_index: graph::NodeIndex<u32>,
        successors: Vec<expansion::Play<RuleSet>>,
    ) -> Result<(), Box<dyn error::Error>> {
        for successor in successors {
            expansion::save_expansion(&mut self.tree, node_index, successor);
        }
        let (to_simulate, state) =
            simulation::fetch_random_child::<RuleSet>(&self.tree, node_index, &mut self.rng);
        let request = simulation::Request::SimulationRequest {
            node_index: to_simulate,
            state,
        };
        self.simulation_request_sender.send(request)?;
        backpropagation::update_tallies(&mut self.tree, to_simulate, true, None);
        Ok(())
    }

    fn wait_for_expansion(&mut self) -> Result<isize, Box<dyn error::Error>> {
        // let response = self.expansion_response_receiver.recv()?;
        // self.handle_expansion(response.node_index, response.successors)?;
        let mut handled = 0;
        loop {
            match self.expansion_response_receiver.try_recv() {
                Ok(response) => {
                    handled += 1;
                    self.handle_expansion(response.node_index, response.successors)?;
                }
                Err(channel::TryRecvError::Empty) => break,
                Err(error) => return Err(Box::new(error)),
            }
        }
        Ok(handled)
    }

    fn wait_for_simulation(&mut self) -> Result<isize, Box<dyn error::Error>> {
        let response = self.simulation_response_receiver.recv()?;
        backpropagation::backpropagate(
            &mut self.tree,
            response.node_index,
            false,
            Some(&response.status),
        );
        let mut handled = 1;
        loop {
            match self.simulation_response_receiver.try_recv() {
                Ok(response) => {
                    handled += 1;
                    backpropagation::backpropagate(
                        &mut self.tree,
                        response.node_index,
                        false,
                        Some(&response.status),
                    );
                }
                Err(channel::TryRecvError::Empty) => break,
                Err(error) => return Err(Box::new(error)),
            }
        }
        Ok(handled)
    }

    fn iterate_sequential(&mut self) -> Result<(), Box<dyn error::Error>> {
        let node = match self.root {
            Some(node) => node,
            None => {
                return Ok(());
            }
        };
        match self.make_selection(node)? {
            SelectionResult::Expansion => {
                self.wait_for_expansion()?;
            }
            SelectionResult::Simulation => (),
            SelectionResult::Nothing => return Ok(()),
        }
        self.wait_for_simulation()?;
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
                requests::Request::IterateSequentially { count } => {
                    for _ in 0..count {
                        self.iterate_sequential()?;
                    }
                }
                requests::Request::IterateParallel {
                    count,
                    expansions_to_do,
                    simulations_to_do,
                } => {
                    self.iterate_concurrent(count, expansions_to_do, simulations_to_do)?;
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
