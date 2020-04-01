#[derive(Debug, PartialEq)]
pub enum Status {
    Ongoing,
    Draw,
    Win{player: u8},
}

pub trait RuleSet {
    type State;
    type Ply: Copy;

    fn initial_state(&self) -> Self::State;
    fn available_plies(&self, state: &Self::State) -> Vec<Self::Ply>;
    fn play(&self, state: &Self::State, ply: &Self::Ply) -> Result<Self::State, PlayError>;
    fn status(&self, state: &Self::State) -> Status;
}

#[derive(Debug)]
pub struct PlayError {
    pub message: &'static str,
    pub field: &'static str,
}
