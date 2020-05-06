use std::fmt;
use std::hash;

pub type Player = u8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Ongoing,
    Draw,
    Win { player: Player },
}

impl Status {
    pub fn player_pov(&self, player: &Player) -> PlayerStatus {
        match self {
            Status::Ongoing => PlayerStatus::Ongoing,
            Status::Draw => PlayerStatus::Draw,
            Status::Win { player: winner } => {
                if winner == player {
                    PlayerStatus::Win
                } else {
                    PlayerStatus::Loss
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PlayerStatus {
    Ongoing,
    Win,
    Draw,
    Loss,
}

pub trait StateTrait:
    Clone + fmt::Debug + Eq + hash::Hash + Ord + PartialEq + PartialOrd + Send
{
    fn current_player(&self) -> Player;
}

pub trait PlyTrait: Copy + fmt::Debug + Send {}

pub trait RuleSetTrait: Send + Sized {
    type State: StateTrait;
    type Ply: PlyTrait;
    type PlyIterator: PlyIteratorTrait<Self>;

    fn initial_state(&self) -> Self::State;
    fn play(&self, state: &Self::State, ply: &Self::Ply) -> Result<Self::State, PlayError>;
    fn status(&self, state: &Self::State) -> Status;
}

pub trait Permutable: RuleSetTrait {
    type Permutation;
    type PermutationIterator: PermutationIteratorTrait<Self>;

    fn swap_state(&self, state: &Self::State, permutation: &Self::Permutation) -> Self::State;
    fn reverse_state(&self, state: &Self::State, permutation: &Self::Permutation) -> Self::State;
}

pub trait PlyIteratorTrait<RuleSet: RuleSetTrait>: Iterator<Item = RuleSet::Ply> {
    fn new(state: RuleSet::State) -> Self;
    fn current_state(&self) -> &RuleSet::State;

    fn next_state(&mut self, ruleset: &RuleSet) -> Option<(RuleSet::Ply, RuleSet::State)> {
        match self.next() {
            Some(ply) => Some((ply, ruleset.play(&self.current_state(), &ply).unwrap())),
            None => None,
        }
    }
}

pub trait PermutationIteratorTrait<Rules: Permutable>: Iterator<Item = Rules::Permutation> {
    fn new(ruleset: &Rules) -> Self;
}

#[derive(Debug)]
pub struct PlayError {
    pub message: &'static str,
    pub field: &'static str,
}
