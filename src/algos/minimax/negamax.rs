use super::state;
use crate::rulesets;
use crate::rulesets::PlyIteratorTrait;
use std::f32;
use std::rc;

pub struct Negamax<RuleSet: rulesets::RuleSetTrait> {
    ruleset: RuleSet,
}

impl<RuleSet: rulesets::RuleSetTrait> Negamax<RuleSet> {
    pub fn compute(&self, state: rc::Rc<RuleSet::State>, player: u8) -> state::State<RuleSet::Ply> {
        self.iterate(state, player, f32::NEG_INFINITY, f32::INFINITY)
    }

    fn iterate(
        &self,
        state: rc::Rc<RuleSet::State>,
        player: u8,
        mut alpha: f32,
        beta: f32,
    ) -> state::State<RuleSet::Ply> {
        match self.ruleset.status(&state) {
            rulesets::Status::Win { player: winner } => {
                if winner == player {
                    state::State::Win
                } else {
                    state::State::Loss
                }
            }
            rulesets::Status::Draw => state::State::Draw,
            rulesets::Status::Ongoing => {
                let available_plies = RuleSet::PlyIterator::new(state.clone());
                let mut current_state = state::State::Unset;
                for ply in available_plies {
                    let resulting_state = rc::Rc::new(self.ruleset.play(&state, &ply).unwrap());
                    let iteration_state = self.iterate(resulting_state, 1 - player, -beta, -alpha);
                    if iteration_state.should_replace(&current_state) {
                        current_state = state::State::tree_search(ply, iteration_state);
                        alpha = alpha.max(current_state.score());
                        if alpha >= beta {
                            break;
                        }
                    }
                }
                current_state
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Negamax;
    use crate::rulesets::ninarow;
    use std::f32;
    use std::rc;

    macro_rules! iterate_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p1_indices, p2_indices, current_player, expected_indices, expected_score) = $value;
                    let ruleset = ninarow::TicTacToe::new();
                    let state = rc::Rc::new(ninarow::TicTacToeState::from_indices(&p1_indices, &p2_indices, current_player));
                    let algo = Negamax{ruleset};
                    let result = algo.compute(state, current_player);
                    assert_eq!(result.score(), expected_score);
                    let expected_plies: Vec<ninarow::Ply> = expected_indices.iter().map(
                        |index| ninarow::Ply{index: *index}
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
