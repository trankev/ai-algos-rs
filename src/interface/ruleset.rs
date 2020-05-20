use super::permutation_iterator;
use super::ply;
use super::ply_iterator;
use super::state;
use super::status;
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

pub trait WithPermutableState: RuleSetTrait {
    type Permutation;
    type PermutationIterator: permutation_iterator::PermutationIteratorTrait<Self>;

    fn swap_state(&self, state: &Self::State, permutation: &Self::Permutation) -> Self::State;
    fn swap_ply(&self, ply: &Self::Ply, permutation: &Self::Permutation) -> Self::Ply;
    fn reverse_state(&self, state: &Self::State, permutation: &Self::Permutation) -> Self::State;
}
