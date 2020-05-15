use super::plies;
use super::ruleset;
use super::state;
use super::variants;
use crate::rulesets;
use crate::utils::grids::strips;
use std::sync;

pub struct PlyIterator<Variant: variants::BaseVariant> {
    state: state::State<Variant>,
    pub strips: sync::Arc<Vec<strips::Indices>>,
}

impl<Variant: variants::BaseVariant> rulesets::PlyIteratorTrait<ruleset::Reversi<Variant>>
    for PlyIterator<Variant>
{
    fn new(ruleset: &ruleset::Reversi<Variant>, state: state::State<Variant>) -> Self {
        PlyIterator::<Variant> {
            state,
            strips: ruleset.strips.clone(),
        }
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
