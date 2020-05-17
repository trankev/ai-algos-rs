use std::fmt;
use std::hash;

pub trait PlyTrait: Copy + fmt::Debug + Send {
    fn ascii_representation(&self) -> String;
}

pub trait ComparablePly: PlyTrait + Eq + hash::Hash + Ord + PartialEq + PartialOrd {}
