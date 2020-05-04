use super::iterator;
use super::requests;
use super::responses;
use crate::rulesets;
use crossbeam::channel;
use std::error;
use std::rc;

pub struct Worker<RuleSet: rulesets::Permutable + 'static> {
    ruleset: RuleSet,
    receiver: channel::Receiver<Option<requests::Request<RuleSet>>>,
    sender: channel::Sender<responses::Response<RuleSet>>,
}

impl<RuleSet: rulesets::Permutable + 'static> Worker<RuleSet> {
    pub fn new(
        ruleset: RuleSet,
        receiver: channel::Receiver<Option<requests::Request<RuleSet>>>,
        sender: channel::Sender<responses::Response<RuleSet>>,
    ) -> Worker<RuleSet> {
        Worker {
            ruleset,
            receiver,
            sender,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn error::Error>> {
        while let Some(requests::Request { node_index, state }) = self.receiver.recv()? {
            let mut iterator = iterator::Expander::new(rc::Rc::new(state));
            let mut successors = Vec::new();
            while let Some(item) = iterator.iterate(&self.ruleset) {
                successors.push(item);
            }
            self.sender.send(responses::Response {
                node_index,
                successors,
            })?;
        }
        Ok(())
    }
}
