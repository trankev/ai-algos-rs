use crate::interface::ai;
use crate::interface::rulesets;
use crate::interface::rulesets::TurnByTurnState;
use std::error;

pub fn play<RuleSet, Player1, Player2>(
    ruleset: &RuleSet,
    player1: &mut Player1,
    player2: &mut Player2,
) -> Result<ai::GameLog<RuleSet>, Box<dyn error::Error>>
where
    RuleSet: rulesets::Deterministic,
    RuleSet::State: rulesets::TurnByTurnState,
    Player1: ai::Policy<RuleSet>,
    Player2: ai::Policy<RuleSet>,
{
    let mut game_log = ai::GameLog::new();
    let mut state = ruleset.initial_state();
    let mut status = ruleset.status(&state);
    while let rulesets::Status::Ongoing = status {
        let ply;
        if state.current_player() == 0 {
            ply = player1.play(&state)?;
        } else {
            ply = player2.play(&state)?;
        }
        let resulting_state = ruleset.play(&state, &ply).unwrap();
        status = ruleset.status(&resulting_state);
        game_log.history.push((state, ply));
        state = resulting_state;
    }
    game_log.status = status;
    Ok(game_log)
}

pub fn self_play<RuleSet, Player>(
    ruleset: &RuleSet,
    player: &mut Player,
) -> Result<ai::GameLog<RuleSet>, Box<dyn error::Error>>
where
    RuleSet: rulesets::Deterministic,
    RuleSet::State: rulesets::TurnByTurnState,
    Player: ai::Policy<RuleSet>,
{
    let mut game_log = ai::GameLog::new();
    let mut state = ruleset.initial_state();
    let mut status = ruleset.status(&state);
    while let rulesets::Status::Ongoing = status {
        let ply = player.play(&state)?;
        let resulting_state = ruleset.play(&state, &ply).unwrap();
        status = ruleset.status(&resulting_state);
        game_log.history.push((state, ply));
        state = resulting_state;
    }
    game_log.status = status;
    Ok(game_log)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::minimax;
    use crate::interface::ai;
    use crate::interface::rulesets;
    use crate::interface::rulesets::Deterministic;
    use crate::interface::rulesets::RuleSetTrait;
    use crate::rulesets::connectn;
    use std::error;

    fn validate_game_log(
        ruleset: &connectn::TicTacToe,
        game_log: &ai::GameLog<connectn::TicTacToe>,
    ) -> Result<(), Box<dyn error::Error>> {
        let mut current_state = ruleset.initial_state();
        for (state, ply) in &game_log.history {
            assert_eq!(current_state, *state);
            let status = ruleset.status(&state);
            assert_eq!(status, rulesets::Status::Ongoing);
            current_state = ruleset.play(&state, &ply).unwrap();
        }
        let status = ruleset.status(&current_state);
        assert_eq!(status, game_log.status);
        Ok(())
    }

    #[test]
    fn test_play() -> Result<(), Box<dyn error::Error>> {
        let ruleset = connectn::TicTacToe::new();
        let mut agent = minimax::Negamax::new(&ruleset);
        let mut opponent = minimax::Negamax::new(&ruleset);
        let game_log = play(&ruleset, &mut agent, &mut opponent)?;
        validate_game_log(&ruleset, &game_log)?;
        Ok(())
    }

    #[test]
    fn test_self_play() -> Result<(), Box<dyn error::Error>> {
        let ruleset = connectn::TicTacToe::new();
        let mut agent = minimax::Negamax::new(&ruleset);
        let game_log = self_play(&ruleset, &mut agent)?;
        validate_game_log(&ruleset, &game_log)?;
        Ok(())
    }
}
