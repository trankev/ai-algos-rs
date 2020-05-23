mod plies;
mod ply_iterators;
mod ruleset;
mod state;
mod symmetry;
mod symmetry_iterators;
mod variants;

pub use plies::Ply;
use ruleset::RuleSet;
pub use state::State;
pub use symmetry::Symmetry;
pub use symmetry_iterators::SymmetryIterator;

pub type TicTacToeState = State<variants::TicTacToe>;
pub type GomokuState = State<variants::Gomoku>;

pub type TicTacToePly = Ply<variants::TicTacToe>;
pub type GomokuPly = Ply<variants::Gomoku>;

pub type TicTacToe = RuleSet<variants::TicTacToe>;
pub type Gomoku = RuleSet<variants::Gomoku>;
