use super::plies;
use super::ruleset;
use super::state;
use super::variants;
use crate::rulesets;

pub struct PlyIterator<Variant: variants::BaseVariant> {
    state: state::State<Variant>,
}

impl<Variant: variants::BaseVariant> rulesets::PlyIteratorTrait<ruleset::Reversi<Variant>>
    for PlyIterator<Variant>
{
    fn new(state: state::State<Variant>) -> Self {
        PlyIterator::<Variant> { state }
    }

    fn current_state(&self) -> &state::State<Variant> {
        &self.state
    }
}

impl<Variant: variants::BaseVariant> Iterator for PlyIterator<Variant> {
    type Item = plies::Ply;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
