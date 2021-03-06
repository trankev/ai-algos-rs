use super::variants;
use crate::interface::rulesets;
use std::marker;

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Ply<Variant: variants::BaseVariant> {
    pub index: u8,
    variant: marker::PhantomData<Variant>,
}

impl<Variant: variants::BaseVariant> Ply<Variant> {
    pub fn new(index: u8) -> Ply<Variant> {
        Ply {
            index,
            variant: marker::PhantomData,
        }
    }
}

impl<Variant: variants::BaseVariant> rulesets::PlyTrait for Ply<Variant> {
    fn ascii_representation(&self) -> String {
        let row = self.index / Variant::GRID_SIZE as u8;
        let column = self.index % Variant::GRID_SIZE as u8;
        format!("[{}, {}]", row, column)
    }
}
