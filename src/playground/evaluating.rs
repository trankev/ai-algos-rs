use crate::interface::ai;
use crate::interface::rulesets;
use crate::playground;
use std::collections;
use std::error;
use std::hash;

pub fn evaluate<RuleSet, Player, Opponent>(
    ruleset: &RuleSet,
    player: &mut Player,
    opponent: &mut Opponent,
    samples: usize,
) -> Result<collections::HashMap<rulesets::Status, usize>, Box<dyn error::Error>>
where
    Player: ai::Agent<RuleSet>,
    Opponent: ai::Agent<RuleSet>,
    RuleSet: rulesets::Deterministic + rulesets::TurnByTurn,
    RuleSet::State: Eq + Ord,
    RuleSet::Ply: hash::Hash + Ord,
{
    let mut scores = collections::HashMap::<rulesets::Status, usize>::new();
    for _ in 0..samples {
        let game_log = playground::play(ruleset, player, opponent)?;
        *scores.entry(game_log.status).or_insert(0) += 1;
    }
    Ok(scores)
}

pub fn self_evaluate<RuleSet, Player>(
    ruleset: &RuleSet,
    player: &mut Player,
    samples: usize,
) -> Result<collections::HashMap<rulesets::Status, usize>, Box<dyn error::Error>>
where
    Player: ai::Agent<RuleSet>,
    RuleSet: rulesets::Deterministic + rulesets::TurnByTurn,
    RuleSet::State: Eq + Ord,
    RuleSet::Ply: hash::Hash + Ord,
{
    let mut scores = collections::HashMap::<rulesets::Status, usize>::new();
    for _ in 0..samples {
        let game_log = playground::self_play(ruleset, player)?;
        *scores.entry(game_log.status).or_insert(0) += 1;
    }
    Ok(scores)
}
