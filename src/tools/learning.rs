use crate::agents::egreedy;
use crate::interface::ai;
use crate::interface::rulesets;
use crate::tools::playing;
use std::collections;
use std::error;
use std::hash;

pub fn train<RuleSet, Player, Opponent>(
    ruleset: &RuleSet,
    player: &mut Player,
    opponent: &mut Opponent,
    exploration_rate: f32,
    samples: usize,
) -> Result<(), Box<dyn error::Error>>
where
    Player: ai::Policy<RuleSet> + ai::Teachable<RuleSet>,
    Opponent: ai::Policy<RuleSet>,
    RuleSet: rulesets::Deterministic + rulesets::EncodableState + rulesets::HasStatesWithSymmetries,
    RuleSet::State: Eq + Ord + rulesets::TurnByTurnState,
    RuleSet::Ply: hash::Hash + Ord,
{
    let mut eagent = egreedy::EGreedy::new(ruleset, exploration_rate, player);
    let mut logs = Vec::new();
    for _ in 0..samples {
        let game_log = playing::play(ruleset, &mut eagent, opponent)?;
        logs.push(game_log);
    }
    player.learn(&logs)?;
    Ok(())
}

pub fn test<RuleSet, Player, Opponent>(
    ruleset: &RuleSet,
    player: &mut Player,
    opponent: &mut Opponent,
    samples: usize,
) -> Result<collections::HashMap<rulesets::Status, usize>, Box<dyn error::Error>>
where
    Player: ai::Policy<RuleSet>,
    Opponent: ai::Policy<RuleSet>,
    RuleSet: rulesets::Deterministic + rulesets::EncodableState + rulesets::HasStatesWithSymmetries,
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
