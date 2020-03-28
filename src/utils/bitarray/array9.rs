use super::BitArray;


#[derive(Clone, Copy, Debug)]
pub struct BitArray9 {
    bits: u16,
}

impl BitArray for BitArray9 {
    type Index = u8;
    fn zero() -> BitArray9 {
        BitArray9 {
            bits: 0,
        }
    }

    fn isset(&self, index: Self::Index) -> bool {
        debug_assert!(index < 9, format!("BitArray index out of bound: {} >= 9", index));
        let mask = 1u16 << index;
        self.bits & mask == mask
    }

    fn set(&mut self, index: Self::Index) {
        self.bits |= 1u16 << index;
    }
}


#[cfg(test)]
mod tests {
    use super::BitArray9;
    use super::super::BitArray;

    #[test]
    fn test_zero() {
        let instance = BitArray9::zero();
        for index in 0..9 {
            assert!(!instance.isset(index));
        }
    }

    #[test]
    #[should_panic]
    fn test_out_of_bound() {
        let instance = BitArray9::zero();
        instance.isset(9);
    }
}
