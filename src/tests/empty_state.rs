use crate::interface;

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct EmptyState {
    current_player: interface::Player,
}

impl EmptyState {
    pub fn new() -> EmptyState {
        EmptyState { current_player: 0 }
    }
}

impl interface::StateTrait for EmptyState {
    fn ascii_representation(&self) -> String {
        "None".into()
    }
}

impl interface::TurnByTurnState for EmptyState {
    fn current_player(&self) -> interface::Player {
        self.current_player
    }
}

impl interface::ComparableState for EmptyState {}
