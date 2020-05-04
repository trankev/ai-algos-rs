use super::algo;
use super::requests;
use super::responses;
use crate::rulesets;
use crossbeam::channel;
use rand;
use rand::rngs;
use std::error;
use std::rc;

pub struct Worker<RuleSet: rulesets::RuleSetTrait> {
    ruleset: RuleSet,
    receiver: channel::Receiver<Option<requests::Request<RuleSet>>>,
    sender: channel::Sender<responses::Response>,
    rng: rngs::ThreadRng,
}

impl<RuleSet: rulesets::RuleSetTrait> Worker<RuleSet> {
    pub fn new(
        ruleset: RuleSet,
        receiver: channel::Receiver<Option<requests::Request<RuleSet>>>,
        sender: channel::Sender<responses::Response>,
    ) -> Worker<RuleSet> {
        Worker {
            ruleset,
            receiver,
            sender,
            rng: rand::thread_rng(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn error::Error>> {
        while let Some(requests::Request { node_index, state }) = self.receiver.recv()? {
            let status = algo::simulate(&self.ruleset, rc::Rc::new(state), &mut self.rng);
            self.sender
                .send(responses::Response { node_index, status })?;
        }
        Ok(())
    }
}
