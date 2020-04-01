use crate::rulesets;
use std::f32;

pub struct Negamax<RuleSet: rulesets::RuleSet> {
    ruleset: RuleSet,
}

impl<RuleSet: rulesets::RuleSet> Negamax<RuleSet> {
    pub fn iterate(&self, state: &RuleSet::State, player: u8) -> (Vec<RuleSet::Ply>, f32) {
        match self.ruleset.status(&state) {
            rulesets::Status::Win{player: winner} => (
                vec![],
                if winner == player { f32::INFINITY } else { f32::NEG_INFINITY },
            ),
            rulesets::Status::Draw => (vec![], 0.0),
            rulesets::Status::Ongoing => {
                let available_plies = self.ruleset.available_plies(&state);
                let mut current_score = f32::NEG_INFINITY;
                let mut current_plies: Option<Vec<RuleSet::Ply>> = None;
                for ply in available_plies {
                    let resulting_state = self.ruleset.play(&state, &ply).unwrap();
                    let (mut ply_list, score) = self.iterate(&resulting_state, 1 - player);
                    if current_plies.is_none() || current_score < -score {
                        current_score = -score;
                        ply_list.push(ply);
                        current_plies = Some(ply_list);
                    }
                }
                (current_plies.unwrap(), current_score)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32;
    use crate::rulesets::tictactoe;
    use crate::rulesets::RuleSet;
    use super::Negamax;

    #[test]
    fn test_sample() {
        let ruleset = tictactoe::TicTacToe::new();
        let initial_state = ruleset.initial_state();
        let algo = Negamax{ruleset};
        let result = algo.iterate(&initial_state, 0);
        println!("{:?}", result);
        assert!(false);
    }

    macro_rules! iterate_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p1_indices, p2_indices, current_player, expected_indices, expected_score) = $value;
                    let ruleset = tictactoe::TicTacToe::new();
                    let state = tictactoe::State::from_indices(&p1_indices, &p2_indices, current_player);
                    let algo = Negamax{ruleset};
                    let (plies, score) = algo.iterate(&state, current_player);
                    assert_eq!(score, expected_score);
                    let expected_plies: Vec<tictactoe::Ply> = expected_indices.iter().map(
                        |index| tictactoe::Ply{index: *index}
                    ).collect();
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
        p1_winning_position: ([4], [5], 0, vec![3, 5], f32::INFINITY),
        hum: ([4, 0], [5], 1, vec![], f32::NEG_INFINITY),
    }
}
