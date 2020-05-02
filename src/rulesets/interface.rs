use std::fmt;
use std::rc;

pub type Player = u8;

#[derive(Debug, PartialEq)]
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

pub trait StateTrait: fmt::Debug {}
pub trait PlyTrait: Copy + fmt::Debug {}

pub trait RuleSetTrait: Sized {
    type State: StateTrait;
    type Ply: PlyTrait;
    type PlyIterator: PlyIteratorTrait<Self>;

    fn initial_state(&self) -> Self::State;
    fn play(&self, state: &Self::State, ply: &Self::Ply) -> Result<Self::State, PlayError>;
    fn status(&self, state: &Self::State) -> Status;
}

pub trait PlyIteratorTrait<Rules: RuleSetTrait>: Iterator<Item = Rules::Ply> {
    fn new(state: rc::Rc<Rules::State>) -> Self;
}

#[derive(Debug)]
pub struct PlayError {
    pub message: &'static str,
    pub field: &'static str,
}
