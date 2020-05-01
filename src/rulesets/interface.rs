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

pub trait BaseRuleSet {
    type State;
    type Ply: Copy;

    fn initial_state(&self) -> Self::State;
    fn play(&self, state: &Self::State, ply: &Self::Ply) -> Result<Self::State, PlayError>;
    fn status(&self, state: &Self::State) -> Status;
}

pub trait PlyIterator<Rules: BaseRuleSet>: Iterator<Item = Rules::Ply> {
    fn new(state: rc::Rc<Rules::State>) -> Self;
}

#[derive(Debug)]
pub struct PlayError {
    pub message: &'static str,
    pub field: &'static str,
}
