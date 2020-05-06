pub mod connectn;
mod interface;

pub use interface::Permutable;
pub use interface::PermutationIteratorTrait;
pub use interface::PlayError;
pub use interface::Player;
pub use interface::PlayerStatus;
pub use interface::PlyIteratorTrait;
pub use interface::PlyTrait;
pub use interface::RuleSetTrait;
pub use interface::StateTrait;
pub use interface::Status;

pub mod tests {
    use super::interface;

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
        fn current_player(&self) -> interface::Player {
            self.current_player
        }
    }
}
