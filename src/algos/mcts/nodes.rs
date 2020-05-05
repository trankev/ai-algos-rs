pub struct Node<State> {
    pub state: State,
    pub visits: f32,
    pub wins: f32,
    pub draws: f32,
}

impl<State> Node<State> {
    pub fn new(state: State) -> Node<State> {
        Node {
            state,
            visits: 0.0,
            wins: 0.0,
            draws: 0.0,
        }
    }

    pub fn score(&self) -> f32 {
        (self.wins + self.draws / 2.0) / (self.visits + 1.0)
    }
}
