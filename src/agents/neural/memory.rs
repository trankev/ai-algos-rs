use crate::interface::rulesets;

pub struct Memory {
    pub states: Vec<f32>,
    pub allowed_plies: Vec<f32>,
    pub actions: Vec<i32>,
    players: Vec<rulesets::Player>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            states: Vec::new(),
            actions: Vec::new(),
            allowed_plies: Vec::new(),
            players: Vec::new(),
        }
    }

    pub fn play(
        &mut self,
        player: rulesets::Player,
        state: &Vec<f32>,
        allowed_plies: &Vec<f32>,
        action: i32,
    ) {
        self.players.push(player);
        self.states.extend(state);
        self.allowed_plies.extend(allowed_plies);
        self.actions.push(action);
    }

    pub fn compute_rewards(&self, status: rulesets::Status, discount_factor: f32) -> Vec<f32> {
        let reward: f32 = match status {
            rulesets::Status::Win { player } => {
                if player == 0 {
                    1.0
                } else {
                    0.0
                }
            }
            rulesets::Status::Draw => 0.5,
            rulesets::Status::Ongoing => unreachable!(),
        };
        let mut rewards = vec![reward; self.actions.len()];
        let mut discounted_reward = reward;
        for index in (0..self.actions.len()).rev() {
            rewards[index] = if self.players[index] == 0 {
                discounted_reward
            } else {
                0.0
            };
            discounted_reward *= discount_factor;
        }
        rewards
    }

    pub fn clear(&mut self) {
        self.states.clear();
        self.actions.clear();
        self.players.clear();
    }
}
