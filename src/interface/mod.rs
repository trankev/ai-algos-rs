mod permutation_iterator;
mod ply;
mod ply_iterator;
mod ruleset;
mod state;
mod status;

pub use permutation_iterator::PermutationIteratorTrait;
pub use ply::PlyTrait;
pub use ply_iterator::PlyIteratorTrait;
pub use ruleset::Deterministic;
pub use ruleset::RuleSetTrait;
pub use ruleset::WithPermutableState;
pub use state::StateTrait;
pub use state::TurnByTurnState;
pub use status::PlayerStatus;
pub use status::Status;

#[derive(Debug)]
pub struct PlayError {
    pub message: &'static str,
    pub field: &'static str,
}

pub type Player = u8;
