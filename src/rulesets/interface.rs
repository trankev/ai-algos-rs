pub enum Status {
    Ongoing,
    Draw,
    Win{player: u8},
}

pub trait RuleSet {
    type State;

    fn initial_state(&self) -> Self::State;
}
