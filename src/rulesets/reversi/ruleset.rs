use super::plies;
use super::ply_iterators;
use super::state;
use super::variants;
use crate::rulesets;
use crate::utils::grids::strips;
use std::marker;
use std::sync;

#[derive(Clone)]
pub struct Reversi<Variant: variants::BaseVariant> {
    variant: marker::PhantomData<Variant>,
    pub strips: sync::Arc<Vec<strips::Indices>>,
}

impl<Variant: variants::BaseVariant> Reversi<Variant> {
    pub fn new() -> Reversi<Variant> {
        Reversi {
            variant: marker::PhantomData,
            strips: sync::Arc::new(
                strips::StripIterator::new(Variant::DIMENSIONS.to_vec()).collect(),
            ),
        }
    }
}

impl<Variant: variants::BaseVariant> rulesets::RuleSetTrait for Reversi<Variant> {
    type Ply = plies::Ply;
    type State = state::State<Variant>;
    type PlyIterator = ply_iterators::PlyIterator<Variant>;

    fn initial_state(&self) -> Self::State {
        state::State::new()
    }

    fn play(
        &self,
        state: &Self::State,
        _ply: &Self::Ply,
    ) -> Result<Self::State, rulesets::PlayError> {
        Ok(state.clone())
    }

    fn status(&self, _state: &Self::State) -> rulesets::Status {
        rulesets::Status::Ongoing
    }
}
