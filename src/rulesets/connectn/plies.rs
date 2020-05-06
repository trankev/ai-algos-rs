use crate::rulesets;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Ply {
    pub index: u8,
}

impl rulesets::PlyTrait for Ply {
    fn ascii_representation(&self) -> String {
        let row = self.index / 3;
        let column = self.index / 3;
        format!("[{}, {}]", row, column)
    }
}
