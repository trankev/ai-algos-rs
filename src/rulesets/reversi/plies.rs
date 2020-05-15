use crate::interface;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Ply {
    Place(usize),
    Pass,
}

impl interface::PlyTrait for Ply {
    fn ascii_representation(&self) -> String {
        match self {
            Ply::Place(index) => format!("Place({})", index),
            Ply::Pass => String::from("Pass"),
        }
    }
}
