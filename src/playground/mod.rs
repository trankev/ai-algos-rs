//! Set of tools to train and test AIs

mod evaluating;
mod playing;
mod training;

pub use evaluating::evaluate;
pub use evaluating::self_evaluate;
pub use playing::play;
pub use playing::self_play;
pub use training::self_train;
pub use training::train;
