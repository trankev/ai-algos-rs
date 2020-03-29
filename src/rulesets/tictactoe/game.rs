use crate::rulesets;
use crate::utils::bitarray;
use crate::utils::bitarray::BitArray;
use super::plies;
use super::state;

pub struct TicTacToe {
    strips: Vec<bitarray::BitArray9>,
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            strips: vec![
                bitarray::BitArray9::from_indices(&[0, 1, 2]),
                bitarray::BitArray9::from_indices(&[3, 4, 5]),
                bitarray::BitArray9::from_indices(&[6, 7, 8]),
                bitarray::BitArray9::from_indices(&[0, 3, 6]),
                bitarray::BitArray9::from_indices(&[1, 4, 7]),
                bitarray::BitArray9::from_indices(&[2, 5, 8]),
                bitarray::BitArray9::from_indices(&[0, 4, 8]),
                bitarray::BitArray9::from_indices(&[2, 4, 6]),
            ]
        }
    }
}

impl rulesets::RuleSet for TicTacToe {
    type State = state::State;
    type Ply = plies::Ply;

    fn initial_state(&self) -> Self::State {
        state::State::new()
    }

    fn available_plies(&self, state: &Self::State) -> Vec<Self::Ply> {
        (0..9).filter(
            |&index| state.isempty(index)
        ).map(
            |index| plies::Ply{index}
        ).collect()
    }

    fn play(&self, state: &Self::State, ply: &Self::Ply) -> Result<Self::State, rulesets::PlayError> {
        let mut result = (*state).clone();
        if let Err(error) = result.play(ply) {
            return Err(error);
        }
        Ok(result)
    }

    fn status(&self, state: &Self::State) -> rulesets::Status {
        let mut ongoing = false;
        let zero = bitarray::BitArray9::zero();
        for strip in &self.strips {
            for player in 0u8..2 {
                if (&state.grids[player as usize] & strip) == *strip {
                    return rulesets::Status::Win{player}
                }
                if (&state.grids[player as usize] & strip) == zero {
                    ongoing = true;
                }
            }
        }
        if ongoing { rulesets::Status::Ongoing } else { rulesets::Status::Draw }
    }
}


#[cfg(test)]
mod tests {
    use super::TicTacToe;
    use super::super::plies;
    use super::super::state;
    use crate::rulesets;
    use crate::rulesets::RuleSet;

    #[test]
    fn test_available_plies_new_game() {
        let game = TicTacToe::new();
        let state = game.initial_state();
        let available_plies = game.available_plies(&state);
        let expected: Vec<plies::Ply> = (0..9).map(|index| plies::Ply{index}).collect();
        assert_eq!(available_plies, expected);
    }

    #[test]
    fn test_available_plies_played_move() {
        let game = TicTacToe::new();
        let state = game.initial_state();
        let ply = plies::Ply{index: 3};
        let available_plies = game.available_plies(&state);
        assert!(available_plies.contains(&ply));
        let resulting_state = game.play(&state, &ply).unwrap();
        let resulting_plies = game.available_plies(&resulting_state);
        assert!(!resulting_plies.contains(&ply));
    }

    #[test]
    fn test_invalid_move() {
        let game = TicTacToe::new();
        let state = game.initial_state();
        let ply = plies::Ply{index: 3};
        let resulting_state = game.play(&state, &ply).unwrap();
        let result = game.play(&resulting_state, &ply);
        assert!(result.is_err());
    }

    macro_rules! status_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p1_indices, p2_indices, current_player, expected) = $value;
                    let game = TicTacToe::new();
                    let state = state::State::from_indices(&p1_indices, &p2_indices, current_player);
                    let status = game.status(&state);
                    assert_eq!(status, expected);
                }
            )*
        }
    }

    status_tests! {
        new_game: ([], [], 0, rulesets::Status::Ongoing),
        ongoing: ([4, 1, 6, 5], [8, 7, 2], 1, rulesets::Status::Ongoing),
        p1_win: ([4, 1, 0, 2], [5, 7, 8], 1, rulesets::Status::Win{player: 0}),
        p2_win: ([1, 2, 5], [4, 0, 8], 0, rulesets::Status::Win{player: 1}),
        draw: ([4, 1, 6, 5], [8, 7, 2, 3], 0, rulesets::Status::Draw),
    }
}
