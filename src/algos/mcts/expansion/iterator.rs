use super::items;
use crate::interface;
use crate::tools::plies;

pub struct Expander<'a, RuleSet: interface::WithPermutableState>
where
    RuleSet::State: interface::ComparableState,
    RuleSet::Ply: interface::ComparablePly,
{
    ply_iterator: plies::PermutationsIterator<'a, RuleSet>,
    ruleset: &'a RuleSet,
    state: &'a RuleSet::State,
}

impl<'a, RuleSet: interface::WithPermutableState> Expander<'a, RuleSet>
where
    RuleSet::State: interface::ComparableState,
    RuleSet::Ply: interface::ComparablePly,
{
    pub fn new(ruleset: &'a RuleSet, state: &'a RuleSet::State) -> Expander<'a, RuleSet> {
        let ply_iterator = plies::PermutationsIterator::new(ruleset, state);
        Expander {
            ply_iterator,
            ruleset,
            state,
        }
    }

    pub fn iterate(&mut self) -> Option<items::Play<RuleSet>> {
        while let Some(ply) = self.ply_iterator.next() {
            let resulting_state = self.ruleset.play(self.state, &ply).unwrap();
            let status = self.ruleset.status(&resulting_state);
            return Some(items::Play {
                ply,
                state: resulting_state,
                status,
            });
        }
        None
    }
}
