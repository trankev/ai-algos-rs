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
    fn play(&self, state: &Self::State, ply: &Self::Ply) -> Result<Self::State, PlayError>;
    fn status(&self, state: &Self::State) -> status::Status;
}
