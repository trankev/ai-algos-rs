use super::Player;
use std::fmt;

pub trait StateTrait: Clone + fmt::Debug + Send {
    fn ascii_representation(&self) -> String;
}

pub trait TurnByTurnState: StateTrait {
    fn current_player(&self) -> Player;
}
