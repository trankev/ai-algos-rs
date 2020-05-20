//! Monte-carlo tree search
//!
//! # References
//!
//! ## Parallelisation
//!
//! * [P-UCT algorithm](https://openreview.net/attachment?id=BJlQtJSKDB&name=original_pdf)
//! * [Structured parallel MCTS](https://arxiv.org/pdf/1704.00325.pdf)
//! * [Parallelization with a MPPA architecture](https://hal.archives-ouvertes.fr/hal-02183609/document)
//! * [Parallel Monte-Carlo tree search](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.159.4373&rep=rep1&type=pdf)
//!
//! ## Variants
//!
//! * [Asymmetric move selection strategies](https://arxiv.org/pdf/1605.02321.pdf)

mod algo;
mod analysis;
mod backpropagation;
mod edges;
mod expansion;
mod nodes;
pub mod puct;
mod selection;
mod simulation;
mod uct_value;

pub use algo::MCTS;
