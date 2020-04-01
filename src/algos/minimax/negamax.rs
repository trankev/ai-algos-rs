use crate::rulesets;
use std::f32;
use super::state;

pub struct Negamax<RuleSet: rulesets::RuleSet> {
    ruleset: RuleSet,
}

impl<RuleSet: rulesets::RuleSet> Negamax<RuleSet> {
    pub fn compute(&self, state: &RuleSet::State, player: u8) -> state::State<RuleSet::Ply> {
        self.iterate(state, player, f32::NEG_INFINITY, f32::INFINITY)
    }

    fn iterate(&self, state: &RuleSet::State, player: u8, mut alpha: f32, beta: f32) -> state::State<RuleSet::Ply> {
        match self.ruleset.status(&state) {
            rulesets::Status::Win{player: winner} => if winner == player { state::State::Win } else { state::State::Loss },
            rulesets::Status::Draw => state::State::Draw,
            rulesets::Status::Ongoing => {
                let mut available_plies = self.ruleset.available_plies(&state);
                let mut current_state = state::State::Unset;
                for ply in available_plies.drain(..) {
                    let resulting_state = self.ruleset.play(&state, &ply).unwrap();
                    let iteration_state = self.iterate(&resulting_state, 1 - player, -beta, -alpha);
                    if iteration_state.should_replace(&current_state) {
                        current_state = state::State::tree_search(ply, iteration_state);
                        alpha = alpha.max(current_state.score());
                        if alpha >= beta {
                            break;
                        }
                    }
                }
                current_state
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32;
    use crate::rulesets::tictactoe;
    use super::Negamax;

    macro_rules! iterate_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p1_indices, p2_indices, current_player, expected_indices, expected_score) = $value;
                    let ruleset = tictactoe::TicTacToe::new();
                    let state = tictactoe::State::from_indices(&p1_indices, &p2_indices, current_player);
                    let algo = Negamax{ruleset};
                    let result = algo.compute(&state, current_player);
                    assert_eq!(result.score(), expected_score);
                    let expected_plies: Vec<tictactoe::Ply> = expected_indices.iter().map(
                        |index| tictactoe::Ply{index: *index}
                    ).collect();
                    let plies = result.plies();
                    assert_eq!(plies, expected_plies);
                }
            )*
        }
    }

    iterate_tests! {
        p1_win_p1_pov: ([4, 1, 0, 2], [5, 7, 8], 0, vec![], f32::INFINITY),
        p1_win_p2_pov: ([4, 1, 0, 2], [5, 7, 8], 1, vec![], f32::NEG_INFINITY),
        p2_win_p1_pov: ([1, 2, 5], [4, 0, 8], 0, vec![], f32::NEG_INFINITY),
        p2_win_p2_pov: ([1, 2, 5], [4, 0, 8], 1, vec![], f32::INFINITY),
        p1_winning_move: ([4, 1, 0], [5, 7, 8], 0, vec![2], f32::INFINITY),
        draw_p1_pov: ([4, 1, 6, 5], [8, 7, 2, 3], 0, vec![], 0.0),
        draw_p2_pov: ([4, 1, 6, 5], [8, 7, 2, 3], 1, vec![], 0.0),
        drawing_game: ([4, 1, 6, 5], [8, 7, 2], 1, vec![3], 0.0),
    }
}
