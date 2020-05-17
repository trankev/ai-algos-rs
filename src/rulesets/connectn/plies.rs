use crate::interface;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ply {
    pub index: u8,
}

impl interface::PlyTrait for Ply {
    fn ascii_representation(&self) -> String {
        let row = self.index / 15;
        let column = self.index % 15;
        format!("[{}, {}]", row, column)
    }
}

impl interface::ComparablePly for Ply {}
