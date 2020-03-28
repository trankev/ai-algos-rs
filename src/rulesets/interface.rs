pub enum Status {
    Ongoing,
    Draw,
    Win{player: u8},
}

pub trait RuleSet {
    type State;
    type Ply;

    fn initial_state(&self) -> Self::State;
    fn available_plies(&self, state: &Self::State) -> Vec<Self::Ply>;
}
