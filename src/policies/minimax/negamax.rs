use super::state;
use crate::interface::ai;
use crate::interface::rulesets;
use crate::tools::plies;
use std::error;
use std::f32;

pub struct Negamax<'a, RuleSet>
where
    RuleSet: rulesets::Deterministic + rulesets::TurnByTurn,
{
    ruleset: &'a RuleSet,
}

impl<'a, RuleSet> Negamax<'a, RuleSet>
where
    RuleSet: rulesets::Deterministic + rulesets::TurnByTurn,
{
    pub fn new(ruleset: &'a RuleSet) -> Negamax<RuleSet> {
        Negamax { ruleset }
    }

    pub fn compute(&self, state: &RuleSet::State) -> state::State<RuleSet::Ply> {
        self.iterate(state, f32::NEG_INFINITY, f32::INFINITY)
    }

    fn iterate(
        &self,
        state: &RuleSet::State,
        mut alpha: f32,
        beta: f32,
    ) -> state::State<RuleSet::Ply> {
        match self.ruleset.status(&state) {
            rulesets::Status::Win { player: winner } => {
                if winner == self.ruleset.current_player(&state) {
                    state::State::Win
                } else {
                    state::State::Loss
                }
            }
            rulesets::Status::Draw => state::State::Draw,
            rulesets::Status::Ongoing => {
                let available_plies = plies::BasicIterator::new(self.ruleset, &state);
                let mut current_state = state::State::Unset;
                for ply in available_plies {
                    let resulting_state = self.ruleset.play(&state, &ply).unwrap();
                    let iteration_state = self.iterate(&resulting_state, -beta, -alpha);
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

impl<'a, RuleSet> ai::Agent<RuleSet> for Negamax<'a, RuleSet>
where
    RuleSet: rulesets::Deterministic + rulesets::TurnByTurn,
{
    fn play(&mut self, state: &RuleSet::State) -> Result<RuleSet::Ply, Box<dyn error::Error>> {
        match self.compute(state) {
            state::State::TreeSearch { ply, .. } => Ok(ply),
            _ => unreachable!(),
        }
    }
}

impl<'a, RuleSet> ai::Policy<RuleSet> for Negamax<'a, RuleSet>
where
    RuleSet: rulesets::Deterministic + rulesets::TurnByTurn,
{
    fn predict(
        &mut self,
        state: &RuleSet::State,
    ) -> Result<ai::Prediction<RuleSet>, Box<dyn error::Error>> {
        match self.compute(state) {
            state::State::TreeSearch { ply, .. } => Ok(ai::Prediction {
                value: 0.0,
                probabilities: vec![(ply, 1.0)],
            }),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Negamax;
    use crate::rulesets::connectn;
    use std::f32;

    macro_rules! iterate_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p1_indices, p2_indices, current_player, expected_indices, expected_score) = $value;
                    let ruleset = connectn::TicTacToe::new();
                    let state = connectn::TicTacToeState::from_indices(&p1_indices, &p2_indices, current_player);
                    let algo = Negamax{ruleset: &ruleset};
                    let result = algo.compute(&state);
                    assert_eq!(result.score(), expected_score);
                    let expected_plies: Vec<connectn::TicTacToePly> = expected_indices.iter().map(
                        |index| connectn::Ply::new(*index)
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
