use super::plies;
use super::ply_iterators;
use super::state;
use super::variants;
use crate::rulesets;
use std::marker;

#[derive(Clone)]
pub struct Reversi<Variant: variants::BaseVariant> {
    variant: marker::PhantomData<Variant>,
}

impl<Variant: variants::BaseVariant> Reversi<Variant> {
    pub fn new() -> Reversi<Variant> {
        Reversi {
            variant: marker::PhantomData,
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
        ply: &Self::Ply,
    ) -> Result<Self::State, rulesets::PlayError> {
        Ok(state.clone())
    }

    fn status(&self, state: &Self::State) -> rulesets::Status {
        rulesets::Status::Ongoing
    }
}
