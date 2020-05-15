use std::fmt;

pub trait PlyTrait: Copy + fmt::Debug + Send {
    fn ascii_representation(&self) -> String;
}
