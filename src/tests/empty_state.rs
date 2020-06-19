use crate::interface::rulesets;

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct EmptyState {
    current_player: rulesets::Player,
}

impl EmptyState {
    pub fn new() -> EmptyState {
        EmptyState { current_player: 0 }
    }
}

impl rulesets::StateTrait for EmptyState {
    fn ascii_representation(&self) -> String {
        "None".into()
    }
}
