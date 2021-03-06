use super::algo;
use super::requests;
use super::responses;
use crate::interface::rulesets;
use crossbeam::channel;
use rand::rngs;
use std::error;

pub struct Worker<RuleSet: rulesets::Deterministic> {
    ruleset: RuleSet,
    receiver: channel::Receiver<requests::Request<RuleSet>>,
    sender: channel::Sender<responses::Response>,
    rng: rngs::ThreadRng,
    pub operation_count: usize,
}

impl<RuleSet: rulesets::Deterministic> Worker<RuleSet> {
    pub fn new(
        ruleset: RuleSet,
        receiver: channel::Receiver<requests::Request<RuleSet>>,
        sender: channel::Sender<responses::Response>,
    ) -> Worker<RuleSet> {
        Worker {
            ruleset,
            receiver,
            sender,
            rng: rand::thread_rng(),
            operation_count: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn error::Error>> {
        while let requests::Request::SimulationRequest { node_index, state } =
            self.receiver.recv()?
        {
            self.operation_count += 1;
            let status = algo::simulate(&self.ruleset, &state, &mut self.rng);
            self.sender
                .send(responses::Response { node_index, status })?;
        }
        Ok(())
    }
}
