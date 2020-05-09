use super::algo;
use super::requests;
use super::responses;
use crate::rulesets;
use crossbeam::channel;
use rand;
use rand::rngs;
use std::error;

pub struct Worker<RuleSet: rulesets::RuleSetTrait> {
    ruleset: RuleSet,
    receiver: channel::Receiver<requests::Request<RuleSet>>,
    sender: channel::Sender<responses::Response>,
    rng: rngs::ThreadRng,
    pub operation_count: usize,
}

impl<RuleSet: rulesets::RuleSetTrait> Worker<RuleSet> {
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
        loop {
            match self.receiver.recv()? {
                requests::Request::SimulationRequest { node_index, state } => {
                    self.operation_count += 1;
                    let status = algo::simulate(&self.ruleset, &state, &mut self.rng);
                    self.sender
                        .send(responses::Response { node_index, status })?;
                }
                requests::Request::Stop => break,
            }
        }
        Ok(())
    }
}
