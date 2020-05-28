use super::ply;
use super::ply_iterator;
use super::state;
use super::status;
use super::symmetry_iterator;
use super::PlayError;

pub trait RuleSetTrait: Clone + Send + Sized {
    type State: state::StateTrait;
    type Ply: ply::PlyTrait;
    type PlyIterator: ply_iterator::PlyIteratorTrait<Self>;

    fn initial_state(&self) -> Self::State;
    fn status(&self, state: &Self::State) -> status::Status;
}

/// Ruleset with deterministic outcome.
/// For a given state, playing the same move will always result in the same state.
pub trait Deterministic: RuleSetTrait {
    fn play(&self, state: &Self::State, ply: &Self::Ply) -> Result<Self::State, PlayError>;
}

pub trait HasStatesWithSymmetries: RuleSetTrait {
    type Symmetry;
    type SymmetryIterator: symmetry_iterator::SymmetryIteratorTrait<Self>;

    fn swap_state(&self, state: &Self::State, permutation: &Self::Symmetry) -> Self::State;
    fn swap_ply(&self, ply: &Self::Ply, permutation: &Self::Symmetry) -> Self::Ply;
    fn reverse_state(&self, state: &Self::State, permutation: &Self::Symmetry) -> Self::State;
}

pub trait EncodableState: RuleSetTrait {
    const STATE_SIZE: usize;
    const PLY_COUNT: usize;

    fn encode_state(&self, state: &Self::State) -> Vec<f32>;
    fn decode_ply(&self, ply_index: usize) -> Self::Ply;
    fn encode_ply(&self, ply: &Self::Ply) -> usize;
}
