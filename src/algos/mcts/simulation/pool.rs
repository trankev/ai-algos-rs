use super::requests;
use super::responses;
use super::worker;
use crate::rulesets;
use crossbeam::channel;
use std::error;
use std::thread;

pub struct Pool<RuleSet: rulesets::RuleSetTrait + 'static> {
    workers: Vec<thread::JoinHandle<usize>>,
    request_receiver: channel::Receiver<requests::Request<RuleSet>>,
    pub request_sender: channel::Sender<requests::Request<RuleSet>>,
    pub response_receiver: channel::Receiver<responses::Response>,
    response_sender: channel::Sender<responses::Response>,
}

impl<RuleSet: rulesets::RuleSetTrait + 'static> Pool<RuleSet> {
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
        let worker_name = format!("mcts-simu-{}", self.workers.len());
        let receiver = self.request_receiver.clone();
        let sender = self.response_sender.clone();
        let handle = thread::Builder::new()
            .name(worker_name)
            .spawn(move || -> usize {
                let mut worker = worker::Worker::new(ruleset, receiver, sender);
                worker.run().unwrap();
                worker.operation_count
            })?;
        self.workers.push(handle);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<usize, Box<dyn error::Error>> {
        for _ in 0..self.workers.len() {
            self.request_sender.send(requests::Request::Stop)?;
        }
        let mut operation_count = 0;
        while let Some(worker) = self.workers.pop() {
            operation_count += worker.join().unwrap();
        }
        Ok(operation_count)
    }
}
