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
use std::rc;

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

    pub fn set_state(&mut self, state: rc::Rc<RuleSet::State>) {
        let index = self.tree.add_node(nodes::Node::new(state));
        self.root = Some(index);
    }

    pub fn iterate(&mut self, player: rulesets::Player) -> Result<(), Box<dyn error::Error>> {
        let node = match self.root {
            Some(node) => node,
            None => {
                return Ok(());
            }
        };
        let selected = selection::select(&self.tree, node, false);
        self.expand(selected)?;
        let (to_simulate, status) = self.simulate(selected)?;

        let player_status = status.player_pov(&player);
        backpropagation::backpropagate(&mut self.tree, to_simulate, &player_status);
        Ok(())
    }

    fn expand(&mut self, node_index: graph::NodeIndex<u32>) -> Result<(), Box<dyn error::Error>> {
        let weight = self.tree.node_weight(node_index).unwrap();
        if weight.visits == 0.0 {
            return Ok(());
        }
        self.expansion_request_sender
            .send(expansion::Request {
                node_index,
                state: (*weight.state).clone(),
            })
            .unwrap();

        let expansion::Response {
            node_index,
            successors,
        } = self.expansion_response_receiver.recv().unwrap();
        for successor in successors {
            let child_index = self
                .tree
                .add_node(nodes::Node::new(rc::Rc::new(successor.state)));
            self.tree
                .add_edge(node_index, child_index, edges::Edge::new(successor.ply));
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
        let state = (*self.tree.node_weight(to_simulate).unwrap().state).clone();
        self.simulation_request_sender
            .send(simulation::Request { node_index, state })?;
        let simulation::Response { node_index, status } =
            self.simulation_response_receiver.recv()?;
        Ok((node_index, status))
    }
}
