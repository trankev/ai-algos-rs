use crate::interface::ai;
use crate::interface::rulesets;
use std::error;

pub fn play<RuleSet: rulesets::Deterministic, Policy: ai::Policy<RuleSet>>(
    ruleset: &RuleSet,
    agent: &mut Policy,
) -> Result<ai::GameLog<RuleSet>, Box<dyn error::Error>> {
    let mut game_log = ai::GameLog::new();
    let mut state = ruleset.initial_state();
    let mut status = ruleset.status(&state);
    while let rulesets::Status::Ongoing = status {
        let ply = agent.play(&state)?;
        state = ruleset.play(&state, &ply).unwrap();
        status = ruleset.status(&state);
        game_log.history.push((state.clone(), ply));
    }
    game_log.status = status;
    Ok(game_log)
}
