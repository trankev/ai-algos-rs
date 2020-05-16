use super::Player;
use std::fmt;
use std::hash;

pub trait StateTrait: Clone + fmt::Debug + Send {
    fn ascii_representation(&self) -> String;
}

pub trait TurnByTurnState: StateTrait {
    fn current_player(&self) -> Player;
}

pub trait ComparableState: StateTrait + Eq + hash::Hash + Ord + PartialEq + PartialOrd {}
