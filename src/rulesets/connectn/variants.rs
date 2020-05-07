pub trait BaseVariant: Clone + Send {
    const GRID_SIZE: usize;
    const CELL_COUNT: usize = Self::GRID_SIZE * Self::GRID_SIZE;
    const RUN_COUNT: usize;
}

#[derive(Clone)]
pub struct Gomoku {}

impl BaseVariant for Gomoku {
    const GRID_SIZE: usize = 15;
    const RUN_COUNT: usize = 5;
}

#[derive(Clone)]
pub struct TicTacToe {}

impl BaseVariant for TicTacToe {
    const GRID_SIZE: usize = 3;
    const RUN_COUNT: usize = 3;
}
