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

    fn reverse_pegs(
        &self,
        state: &mut state::State<Variant>,
        mut start: isize,
        direction: isize,
        length: isize,
    ) -> bool {
        start += direction;
        let current_player = state.current_player as usize;
        let opponent = 1 - current_player;
        if !state.grids[opponent].isset(start as usize) {
            return false;
        }
        let mut current = start;
        let mut remaining = length;
        loop {
            current += direction;
            if state.grids[current_player].isset(current as usize) {
                break;
            } else if !state.grids[opponent].isset(current as usize) {
                return false;
            }
            remaining -= 1;
            if remaining == 0 {
                return false;
            }
        }
        current = start;
        remaining = length;
        loop {
            if !state.grids[opponent].isset(current as usize) {
                break;
            }
            state.grids[opponent].unset(current as usize);
            state.grids[current_player].set(current as usize);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
            current += direction;
        }
        true
    }
}

impl<Variant: variants::BaseVariant> interface::RuleSetTrait for Reversi<Variant> {
    type Ply = plies::Ply;
    type State = state::State<Variant>;
    type PlyIterator = ply_iterators::PlyIterator<Variant>;

    fn initial_state(&self) -> Self::State {
        state::State::new()
    }

    fn status(&self, state: &Self::State) -> interface::Status {
        let mut ply_iterator = Self::PlyIterator::new(&self, &state);
        match ply_iterator.iterate(&self, &state) {
            Some(_) => interface::Status::Ongoing,
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

impl<Variant: variants::BaseVariant> interface::Deterministic for Reversi<Variant> {
    fn play(
        &self,
        state: &Self::State,
        ply: &Self::Ply,
    ) -> Result<Self::State, interface::PlayError> {
        match ply {
            plies::Ply::Place(index) => {
                if state.grids.iter().any(|grid| grid.isset(*index)) {
                    return Err(interface::PlayError {
                        message: "Cell is occupied",
                        field: "index",
                    });
                }
                let mut result = state.clone();
                let mut found_update = false;
                let index = *index as isize;
                for strip in self.strips.iter() {
                    if strip.start > index || (index - strip.start) % strip.step != 0 {
                        continue;
                    }
                    let distance = (index - strip.start) / strip.step;
                    if distance >= strip.length {
                        continue;
                    }
                    if distance > 1 {
                        found_update |=
                            self.reverse_pegs(&mut result, index, -strip.step, distance);
                    }
                    if strip.length - distance > 1 {
                        found_update |= self.reverse_pegs(
                            &mut result,
                            index,
                            strip.step,
                            strip.length - distance,
                        );
                    }
                }
                if !found_update {
                    return Err(interface::PlayError {
                        message: "No reversal resulting from the peg placement",
                        field: "index",
                    });
                }
                result.grids[result.current_player as usize].set(index as usize);
                result.current_player = 1 - result.current_player;
                Ok(result)
            }
            plies::Ply::Pass => Ok(state.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::instances;
    use super::super::plies;
    use super::*;
    use crate::interface;
    use crate::interface::Deterministic;
    use crate::interface::RuleSetTrait;

    #[test]
    fn test_play_on_occupied_cell() {
        let game = Reversi::<instances::Mini>::new();
        let state = state::State::from_indices(&[5, 10], &[6, 9], 0);
        let ply = plies::Ply::Place(5);
        let result = game.play(&state, &ply);
        assert!(result.is_err());
    }

    #[test]
    fn test_play_on_non_reversing_cell() {
        let game = Reversi::<instances::Mini>::new();
        let state = state::State::from_indices(&[5, 10], &[6, 9], 0);
        let ply = plies::Ply::Place(4);
        let result = game.play(&state, &ply);
        assert!(result.is_err());
    }

    macro_rules! play_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (in_p1_idx, in_p2_idx, in_player, ply_idx, out_p1_idx, out_p2_idx, out_player) = $value;
                    let game = Reversi::<instances::Mini>::new();
                    let state = state::State::from_indices(&in_p1_idx, &in_p2_idx, in_player);
                    let ply = plies::Ply::Place(ply_idx);
                    let resulting_state = game.play(&state, &ply).unwrap();
                    let expected = state::State::from_indices(&out_p1_idx, &out_p2_idx, out_player);
                    assert_eq!(resulting_state, expected);
                }
            )*
        }
    }

    play_tests! {
        // leftward_take: ([5, 10], [6, 9], 0, 7, [5, 6, 7, 10], [9], 1),
        // rightward_take: ([5, 10], [6, 9], 0, 8, [5, 8, 9, 10], [6], 1),
        // upward_take: ([5, 10], [6, 9], 0, 2, [2, 5, 6, 10], [9], 1),
        downward_take: ([5, 10], [6, 9], 0, 13, [5, 9, 10, 13], [6], 1),
        // upleftward_take: ([12], [9], 0, 6, [6, 9, 12], [], 1),
        // uprightward_take: ([15], [10], 0, 5, [5, 10, 15], [], 1),
        // downrightward_take: ([6], [9], 0, 12, [6, 9, 12], [], 1),
        // downleftward_take: ([5], [10], 0, 15, [5, 10, 15], [], 1),
        // several_take: ([2, 8, 10], [1, 4, 5], 0, 0, [0, 1, 2, 4, 5, 8, 10], [], 1),
    }

    macro_rules! status_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p1_indices, p2_indices, current_player, expected) = $value;
                    let game = Reversi::<instances::Mini>::new();
                    let state = state::State::from_indices(&p1_indices, &p2_indices, current_player);
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
