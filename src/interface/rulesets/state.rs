use std::fmt;

pub trait StateTrait: Clone + fmt::Debug + Send {
    fn ascii_representation(&self) -> String;
}
