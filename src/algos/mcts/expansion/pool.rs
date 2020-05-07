use super::requests;
use super::responses;
use super::worker;
use crate::rulesets;
use crossbeam::channel;
use std::error;
use std::thread;

pub struct Pool<RuleSet: rulesets::Permutable + 'static> {
    workers: Vec<thread::JoinHandle<()>>,
    request_receiver: channel::Receiver<requests::Request<RuleSet>>,
    pub request_sender: channel::Sender<requests::Request<RuleSet>>,
    pub response_receiver: channel::Receiver<responses::Response<RuleSet>>,
    response_sender: channel::Sender<responses::Response<RuleSet>>,
}

impl<RuleSet: rulesets::Permutable + 'static> Pool<RuleSet> {
    pub fn new() -> Pool<RuleSet> {
        let (request_sender, request_receiver) = channel::unbounded();
        let (response_sender, response_receiver) = channel::unbounded();
        Pool {
            request_receiver,
            request_sender,
            response_receiver,
            response_sender,
            workers: Vec::new(),
        }
    }

    pub fn spawn(&mut self, ruleset: RuleSet) -> Result<(), Box<dyn error::Error>> {
        let worker_name = format!("mcts-expansion-{}", self.workers.len());
        let receiver = self.request_receiver.clone();
        let sender = self.response_sender.clone();
        let handle = thread::Builder::new().name(worker_name).spawn(move || {
            let mut worker = worker::Worker::new(ruleset, receiver, sender);
            worker.run().unwrap();
        })?;
        self.workers.push(handle);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn error::Error>> {
        self.request_sender.send(requests::Request::Stop)?;
        while let Some(worker) = self.workers.pop() {
            worker.join().unwrap();
        }
        Ok(())
    }
}
