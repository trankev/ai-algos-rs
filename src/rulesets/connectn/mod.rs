mod permutation;
mod permutation_iterators;
mod plies;
mod ply_iterators;
mod ruleset;
mod state;
mod variants;

pub use permutation::Permutation;
pub use permutation_iterators::PermutationIterator;
pub use plies::Ply;
pub use ruleset::Gomoku;
use ruleset::RuleSet;
pub use ruleset::TicTacToe;
pub use state::State;

pub type TicTacToeState = State<variants::TicTacToe>;
pub type GomokuState = State<variants::Gomoku>;
