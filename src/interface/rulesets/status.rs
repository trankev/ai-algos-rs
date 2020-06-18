use super::Player;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PlayerStatus {
    Ongoing,
    Win,
    Draw,
    Loss,
}
