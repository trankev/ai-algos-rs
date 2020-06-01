use crate::interface::ai;
use crate::interface::rulesets;
use crate::tools::playing;
use std::collections;
use std::error;
use std::hash;

pub fn test<RuleSet, Player, Opponent>(
    ruleset: &RuleSet,
    player: &mut Player,
    opponent: &mut Opponent,
    samples: usize,
) -> Result<collections::HashMap<rulesets::Status, usize>, Box<dyn error::Error>>
where
    Player: ai::Policy<RuleSet>,
    Opponent: ai::Policy<RuleSet>,
    RuleSet: rulesets::Deterministic,
    RuleSet::State: Eq + Ord + rulesets::TurnByTurnState,
    RuleSet::Ply: hash::Hash + Ord,
{
    let mut scores = collections::HashMap::<rulesets::Status, usize>::new();
    for _ in 0..samples {
        let game_log = playing::play(ruleset, player, opponent)?;
        *scores.entry(game_log.status).or_insert(0) += 1;
    }
    Ok(scores)
}

pub fn self_test<RuleSet, Player>(
    ruleset: &RuleSet,
    player: &mut Player,
    samples: usize,
) -> Result<collections::HashMap<rulesets::Status, usize>, Box<dyn error::Error>>
where
    Player: ai::Policy<RuleSet>,
    RuleSet: rulesets::Deterministic,
    RuleSet::State: Eq + Ord + rulesets::TurnByTurnState,
    RuleSet::Ply: hash::Hash + Ord,
{
    let mut scores = collections::HashMap::<rulesets::Status, usize>::new();
    for _ in 0..samples {
        let game_log = playing::self_play(ruleset, player)?;
        *scores.entry(game_log.status).or_insert(0) += 1;
    }
    Ok(scores)
}
