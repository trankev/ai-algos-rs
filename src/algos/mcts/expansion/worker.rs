use super::iterator;
use super::requests;
use super::responses;
use crate::interface;
use crossbeam::channel;
use std::error;
use std::hash;

pub struct Worker<RuleSet>
where
    RuleSet: interface::WithPermutableState + interface::Deterministic + 'static,
    RuleSet::Ply: Eq + Ord + hash::Hash,
    RuleSet::State: Eq,
{
    ruleset: RuleSet,
    receiver: channel::Receiver<requests::Request<RuleSet>>,
    sender: channel::Sender<responses::Response<RuleSet>>,
    pub operation_count: usize,
}

impl<RuleSet> Worker<RuleSet>
where
    RuleSet: interface::WithPermutableState + interface::Deterministic + 'static,
    RuleSet::Ply: Eq + Ord + hash::Hash,
    RuleSet::State: Eq,
{
    pub fn new(
        ruleset: RuleSet,
        receiver: channel::Receiver<requests::Request<RuleSet>>,
        sender: channel::Sender<responses::Response<RuleSet>>,
    ) -> Worker<RuleSet> {
        Worker {
            ruleset,
            receiver,
            sender,
            operation_count: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn error::Error>> {
        loop {
            match self.receiver.recv()? {
                requests::Request::ExpansionRequest { node_index, state } => {
                    self.operation_count += 1;
                    let mut iterator = iterator::Expander::new(&self.ruleset, &state);
                    let mut successors = Vec::new();
                    while let Some(item) = iterator.iterate() {
                        successors.push(item);
                    }
                    self.sender.send(responses::Response {
                        node_index,
                        successors,
                    })?;
                }
                requests::Request::Stop => break,
            }
        }
        Ok(())
    }
}
