use crate::rulesets;

#[derive(Clone, Copy, Debug)]
pub enum Ply {
    Place(usize),
    Pass,
}

impl rulesets::PlyTrait for Ply {
    fn ascii_representation(&self) -> String {
        match self {
            Ply::Place(index) => format!("Place({})", index),
            Ply::Pass => String::from("Pass"),
        }
    }
}
