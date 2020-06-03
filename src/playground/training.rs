use crate::interface::ai;
use crate::interface::rulesets;
use crate::playground;
use std::error;
use std::hash;

pub fn train<RuleSet, Player, Opponent>(
    ruleset: &RuleSet,
    player: &mut Player,
    opponent: &mut Opponent,
    samples: usize,
) -> Result<Player::Metrics, Box<dyn error::Error>>
where
    Player: ai::Policy<RuleSet> + ai::Teachable<RuleSet>,
    Opponent: ai::Policy<RuleSet>,
    RuleSet: rulesets::Deterministic + rulesets::EncodableState + rulesets::HasStatesWithSymmetries,
    RuleSet::State: Eq + Ord + rulesets::TurnByTurnState,
    RuleSet::Ply: hash::Hash + Ord,
{
    let mut logs = Vec::new();
    for _ in 0..samples {
        let game_log = playground::play(ruleset, player, opponent)?;
        logs.push(game_log);
    }
    let metrics = player.learn(&logs)?;
    Ok(metrics)
}

pub fn self_train<RuleSet, Player>(
    ruleset: &RuleSet,
    player: &mut Player,
    samples: usize,
) -> Result<Player::Metrics, Box<dyn error::Error>>
where
    Player: ai::Policy<RuleSet> + ai::Teachable<RuleSet>,
    RuleSet: rulesets::Deterministic + rulesets::EncodableState + rulesets::HasStatesWithSymmetries,
    RuleSet::State: Eq + Ord + rulesets::TurnByTurnState,
    RuleSet::Ply: hash::Hash + Ord,
{
    let mut logs = Vec::new();
    for _ in 0..samples {
        let game_log = playground::self_play(ruleset, player)?;
        logs.push(game_log);
    }
    let metrics = player.learn(&logs)?;
    Ok(metrics)
}
