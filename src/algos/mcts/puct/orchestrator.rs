use super::super::expansion;
use super::super::simulation;
use super::master;
use super::requests;
use super::responses;
use crate::algos;
use crate::interface;
use crossbeam::channel;
use std::error;
use std::mem;
use std::thread;

pub struct Orchestrator<RuleSet: interface::Permutable + 'static> {
    ruleset: RuleSet,
    master_handle: Option<thread::JoinHandle<()>>,
    master_request_sender: channel::Sender<requests::Request<RuleSet>>,
    master_request_receiver: channel::Receiver<requests::Request<RuleSet>>,
    master_response_sender: channel::Sender<responses::Response<RuleSet>>,
    master_response_receiver: channel::Receiver<responses::Response<RuleSet>>,
    expansion_pool: expansion::Pool<RuleSet>,
    simulation_pool: simulation::Pool<RuleSet>,
}

impl<RuleSet: interface::Permutable + 'static> Orchestrator<RuleSet> {
    pub fn new(ruleset: RuleSet) -> Orchestrator<RuleSet> {
        let expansion_pool = expansion::Pool::new();
        let simulation_pool = simulation::Pool::new();
        let (master_request_sender, master_request_receiver) = channel::unbounded();
        let (master_response_sender, master_response_receiver) = channel::unbounded();
        Orchestrator {
            ruleset,
            master_request_receiver,
            master_request_sender,
            master_response_receiver,
            master_response_sender,
            expansion_pool,
            simulation_pool,
            master_handle: None,
        }
    }

    pub fn set_state(&self, state: RuleSet::State) -> Result<(), Box<dyn error::Error>> {
        let request = requests::Request::SetState(state);
        self.master_request_sender.send(request)?;
        Ok(())
    }

    pub fn iterate_sequentially(&self, count: usize) -> Result<(), Box<dyn error::Error>> {
        let request = requests::Request::IterateSequentially { count };
        self.master_request_sender.send(request)?;
        Ok(())
    }

    pub fn iterate_parallel(
        &self,
        count: usize,
        expansions_to_do: usize,
        simulations_to_do: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        let request = requests::Request::IterateParallel {
            count,
            expansions_to_do,
            simulations_to_do,
        };
        self.master_request_sender.send(request)?;
        Ok(())
    }

    pub fn ply_considerations(
        &self,
    ) -> Result<Option<Vec<algos::PlyConsideration<RuleSet::Ply>>>, Box<dyn error::Error>> {
        let request = requests::Request::ListConsiderations;
        self.master_request_sender.send(request)?;
        let response = self.master_response_receiver.recv()?;
        if let responses::Response::PlyConsiderations(considerations) = response {
            return Ok(Some(considerations));
        }
        Ok(None)
    }

    pub fn start(
        &mut self,
        expansion_workers: usize,
        simulation_workers: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        for _ in 0..expansion_workers {
            self.expansion_pool.spawn(self.ruleset.clone())?;
        }
        for _ in 0..simulation_workers {
            self.simulation_pool.spawn(self.ruleset.clone())?;
        }
        self.spawn_master()?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(usize, usize), Box<dyn error::Error>> {
        self.stop_master()?;
        let simulation_count = self.simulation_pool.stop()?;
        let expansion_count = self.expansion_pool.stop()?;
        Ok((expansion_count, simulation_count))
    }

    fn stop_master(&mut self) -> Result<(), Box<dyn error::Error>> {
        let request = requests::Request::Stop;
        self.master_request_sender.send(request)?;
        let master_handle = mem::replace(&mut self.master_handle, None);
        if let Some(handle) = master_handle {
            handle.join().unwrap();
        }
        Ok(())
    }

    fn spawn_master(&mut self) -> Result<(), Box<dyn error::Error>> {
        let worker_name = "mcts-master";
        let ruleset = self.ruleset.clone();
        let master_request_receiver = self.master_request_receiver.clone();
        let master_response_sender = self.master_response_sender.clone();
        let expansion_request_sender = self.expansion_pool.request_sender.clone();
        let expansion_response_receiver = self.expansion_pool.response_receiver.clone();
        let simulation_request_sender = self.simulation_pool.request_sender.clone();
        let simulation_response_receiver = self.simulation_pool.response_receiver.clone();
        let handle = thread::Builder::new()
            .name(worker_name.to_string())
            .spawn(move || {
                let mut master = master::Master::new(
                    ruleset,
                    master_request_receiver,
                    master_response_sender,
                    expansion_request_sender,
                    expansion_response_receiver,
                    simulation_request_sender,
                    simulation_response_receiver,
                );
                master.run().unwrap();
            })?;
        self.master_handle = Some(handle);
        Ok(())
    }
}
