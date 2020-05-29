pub struct ReplayBuffer {
    pub states: Vec<f32>,
    pub allowed_plies: Vec<f32>,
    pub plies: Vec<i32>,
    pub rewards: Vec<f32>,
}

impl ReplayBuffer {
    pub fn new() -> ReplayBuffer {
        ReplayBuffer {
            states: Vec::new(),
            allowed_plies: Vec::new(),
            plies: Vec::new(),
            rewards: Vec::new(),
        }
    }
}
