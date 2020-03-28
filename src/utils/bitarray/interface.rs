pub trait BitArray: Clone + Copy {
    type Index;
    fn zero() -> Self;
    fn isset(&self, index: Self::Index) -> bool;
}
