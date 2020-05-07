use super::super::expansion;
use super::super::simulation;
use super::master;
use crate::rulesets;
use std::error;
use std::mem;
use std::thread;

pub struct Orchestrator<RuleSet: rulesets::Permutable + 'static> {
    ruleset: RuleSet,
    master_handle: Option<thread::JoinHandle<()>>,
    expansion_pool: expansion::Pool<RuleSet>,
    simulation_pool: simulation::Pool<RuleSet>,
}

impl<RuleSet: rulesets::Permutable + 'static> Orchestrator<RuleSet> {
    pub fn new(ruleset: RuleSet) -> Orchestrator<RuleSet> {
        let expansion_pool = expansion::Pool::new();
        let simulation_pool = simulation::Pool::new();
        Orchestrator {
            ruleset,
            expansion_pool,
            simulation_pool,
            master_handle: None,
        }
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

    pub fn stop(&mut self) -> Result<(), Box<dyn error::Error>> {
        let master_handle = mem::replace(&mut self.master_handle, None);
        if let Some(handle) = master_handle {
            handle.join().unwrap();
        }
        self.simulation_pool.stop()?;
        self.expansion_pool.stop()?;
        Ok(())
    }

    fn spawn_master(&mut self) -> Result<(), Box<dyn error::Error>> {
        let worker_name = "mcts-master";
        let initial_state = self.ruleset.initial_state();
        let status = self.ruleset.status(&initial_state);
        let expansion_request_sender = self.expansion_pool.request_sender.clone();
        let expansion_response_receiver = self.expansion_pool.response_receiver.clone();
        let simulation_request_sender = self.simulation_pool.request_sender.clone();
        let simulation_response_receiver = self.simulation_pool.response_receiver.clone();
        let handle = thread::Builder::new()
            .name(worker_name.to_string())
            .spawn(move || {
                let mut master = master::Master::new(
                    expansion_request_sender,
                    expansion_response_receiver,
                    simulation_request_sender,
                    simulation_response_receiver,
                );
                master.set_state(initial_state, status);
                master.first_iteration().unwrap();
            })?;
        self.master_handle = Some(handle);
        Ok(())
    }
}
