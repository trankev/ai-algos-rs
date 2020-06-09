mod ply;
mod ply_iterator;
mod ruleset;
mod state;
mod status;
mod symmetry_iterator;

pub use ply::PlyTrait;
pub use ply_iterator::PlyIteratorTrait;
pub use ruleset::Deterministic;
pub use ruleset::EncodableState;
pub use ruleset::HasStatesWithSymmetries;
pub use ruleset::RuleSetTrait;
pub use ruleset::TurnByTurn;
pub use state::StateTrait;
pub use status::PlayerStatus;
pub use status::Status;
pub use symmetry_iterator::SymmetryIteratorTrait;

#[derive(Debug)]
pub struct PlayError {
    pub message: &'static str,
    pub field: &'static str,
}

pub type Player = u8;
