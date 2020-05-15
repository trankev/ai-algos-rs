use super::Player;
use std::fmt;
use std::hash;

pub trait StateTrait:
    Clone + fmt::Debug + Eq + hash::Hash + Ord + PartialEq + PartialOrd + Send
{
    fn current_player(&self) -> Player;
    fn ascii_representation(&self) -> String;
}
