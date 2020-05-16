use super::plies;
use super::ply_iterators;
use super::state;
use super::variants;
use crate::interface;
use crate::interface::PlyIteratorTrait;
use crate::utils::grids::strips;
use std::cmp;
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

impl<Variant: variants::BaseVariant> interface::RuleSetTrait for Reversi<Variant> {
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
    ) -> Result<Self::State, interface::PlayError> {
        Ok(state.clone())
    }

    fn status(&self, state: &Self::State) -> interface::Status {
        let mut ply_iterator = Self::PlyIterator::new(&self, &state);
        match ply_iterator.iterate_grid(&state) {
            Some(ply) => interface::Status::Ongoing,
            None => match state.grids[0]
                .count_ones()
                .cmp(&state.grids[1].count_ones())
            {
                cmp::Ordering::Less => interface::Status::Win { player: 1 },
                cmp::Ordering::Equal => interface::Status::Draw,
                cmp::Ordering::Greater => interface::Status::Win { player: 0 },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::instances;
    use super::*;
    use crate::interface;
    use crate::interface::RuleSetTrait;
    use crate::interface::StateTrait;

    macro_rules! status_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p1_indices, p2_indices, current_player, expected) = $value;
                    let game = Reversi::<instances::Mini>::new();
                    let state = state::State::from_indices(&p1_indices, &p2_indices, current_player);
                    println!("{}", state.ascii_representation());
                    let status = game.status(&state);
                    assert_eq!(status, expected);
                }
            )*
        }
    }

    status_tests! {
        initial_state: ([5, 10], [6, 9], 0, interface::Status::Ongoing),
        p1_win: ([0, 1, 2, 4, 4, 5, 7, 8], [13, 14, 15], 0, interface::Status::Win{player: 0}),
        p2_win: ([0, 1, 4], [3, 6, 7, 9, 11, 12, 13, 14], 0, interface::Status::Win{player: 1}),
        draw: ([1, 2, 3, 4, 6], [9, 11, 12, 13, 14], 0, interface::Status::Draw),
    }
}
