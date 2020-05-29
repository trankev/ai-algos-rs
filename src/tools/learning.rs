use crate::agents::egreedy;
use crate::interface::ai;
use crate::interface::rulesets;
use crate::tools::playing;
use std::collections;
use std::error;
use std::hash;

pub fn train<RuleSet, Policy>(
    ruleset: &RuleSet,
    agent: &mut Policy,
    exploration_rate: f32,
    samples: usize,
) -> Result<(), Box<dyn error::Error>>
where
    Policy: ai::Policy<RuleSet> + ai::Teachable<RuleSet>,
    RuleSet: rulesets::Deterministic + rulesets::EncodableState + rulesets::HasStatesWithSymmetries,
    RuleSet::State: Eq + Ord + rulesets::TurnByTurnState,
    RuleSet::Ply: hash::Hash + Ord,
{
    let mut eagent = egreedy::EGreedy::new(ruleset, exploration_rate, agent);
    let mut logs = Vec::new();
    for _ in 0..samples {
        let game_log = playing::play(ruleset, &mut eagent)?;
        logs.push(game_log);
    }
    agent.learn(&logs)?;
    Ok(())
}

pub fn test<RuleSet, Policy>(
    ruleset: &RuleSet,
    agent: &mut Policy,
    samples: usize,
) -> Result<collections::HashMap<rulesets::Status, usize>, Box<dyn error::Error>>
where
    Policy: ai::Policy<RuleSet>,
    RuleSet: rulesets::Deterministic + rulesets::EncodableState + rulesets::HasStatesWithSymmetries,
    RuleSet::State: Eq + Ord + rulesets::TurnByTurnState,
    RuleSet::Ply: hash::Hash + Ord,
{
    let mut scores = collections::HashMap::<rulesets::Status, usize>::new();
    for _ in 0..samples {
        let game_log = playing::play(ruleset, agent)?;
        *scores.entry(game_log.status).or_insert(0) += 1;
    }
    Ok(scores)
}
