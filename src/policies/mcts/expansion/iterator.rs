use super::items;
use crate::interface::rulesets;
use crate::tools::plies;
use std::hash;

pub struct Expander<'a, RuleSet>
where
    RuleSet: rulesets::HasStatesWithSymmetries + rulesets::Deterministic + rulesets::TurnByTurn,
    RuleSet::Ply: Eq + Ord + hash::Hash,
    RuleSet::State: Eq,
{
    ply_iterator: plies::SymmetriesIterator<'a, RuleSet>,
    ruleset: &'a RuleSet,
    state: &'a RuleSet::State,
}

impl<'a, RuleSet> Expander<'a, RuleSet>
where
    RuleSet: rulesets::HasStatesWithSymmetries + rulesets::Deterministic + rulesets::TurnByTurn,
    RuleSet::Ply: Eq + Ord + hash::Hash,
    RuleSet::State: Eq,
{
    pub fn new(ruleset: &'a RuleSet, state: &'a RuleSet::State) -> Expander<'a, RuleSet> {
        let ply_iterator = plies::SymmetriesIterator::new(ruleset, state);
        Expander {
            ply_iterator,
            ruleset,
            state,
        }
    }

    pub fn iterate(&mut self) -> Option<items::Play<RuleSet>> {
        if let Some(ply) = self.ply_iterator.next() {
            let resulting_state = self.ruleset.play(self.state, &ply).unwrap();
            let status = self.ruleset.status(&resulting_state);
            let current_player = self.ruleset.current_player(&resulting_state);
            return Some(items::Play {
                ply,
                state: resulting_state,
                status,
                current_player,
            });
        }
        None
    }
}
