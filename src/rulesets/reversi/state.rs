use crate::rulesets;
use crate::utils::bitarray;

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct State<ArraySettings: bitarray::BitArraySettings> {
    grids: [bitarray::BitArray<ArraySettings>; 2],
    current_player: rulesets::Player,
}

impl<ArraySettings: bitarray::BitArraySettings> rulesets::StateTrait for State<ArraySettings> {
    fn current_player(&self) -> rulesets::Player {
        self.current_player
    }

    fn ascii_representation(&self) -> String {
        let mut result = String::new();
        for index in 0..64 {
            if self.grids[0].isset(index) {
                result.push('X');
            } else if self.grids[1].isset(index) {
                result.push('O');
            } else {
                result.push('.');
            }
            if index % 8 == 7 {
                result.push('\n');
            }
        }
        format!("{}\nTo play: {}", result, self.current_player())
    }
}
