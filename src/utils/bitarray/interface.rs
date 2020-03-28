pub trait BitArray: Clone + Copy {
    fn zero() -> Self;
    fn isset(&self, index: usize) -> bool;
}
