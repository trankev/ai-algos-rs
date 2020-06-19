use super::variants;
use crate::interface::rulesets;
use std::marker;

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub enum Ply<Variant: variants::BaseVariant> {
    Place(usize),
    Pass,
    Unused(marker::PhantomData<Variant>),
}

impl<Variant: variants::BaseVariant> rulesets::PlyTrait for Ply<Variant> {
    fn ascii_representation(&self) -> String {
        match self {
            Ply::Place(index) => {
                let row = index / Variant::GRID_SIZE;
                let column = index % Variant::GRID_SIZE;
                format!("Place[{}, {}]", row, column)
            }
            Ply::Pass => String::from("Pass"),
            Ply::Unused(_) => unreachable!(),
        }
    }
}
