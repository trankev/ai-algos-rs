use std::ops;
use super::BitArray;


#[derive(Clone, Copy, Debug, PartialEq)]
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

    fn from_indices(indices: &[Self::Index]) -> Self {
        let mut result = BitArray9::zero();
        for index in indices {
            result.set(*index);
        }
        result
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

impl ops::BitAnd<BitArray9> for BitArray9 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        BitArray9 {
            bits: self.bits & rhs.bits
        }
    }
}

impl ops::BitOr<BitArray9> for BitArray9 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        BitArray9 {
            bits: self.bits | rhs.bits
        }
    }
}

impl ops::BitXor<BitArray9> for BitArray9 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        BitArray9 {
            bits: self.bits ^ rhs.bits
        }
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

    #[test]
    fn test_from_positions() {
        let indices = vec![2, 3, 5];
        let instance = BitArray9::from_indices(&indices);
        for index in 0..9 {
            if indices.contains(&index) {
                assert!(instance.isset(index));
            } else {
                assert!(!instance.isset(index));
            }
        }
    }

    #[test]
    fn test_bitor() {
        let array1 = BitArray9::from_indices(&[2, 3, 5]);
        let array2 = BitArray9::from_indices(&[2, 4, 6]);
        let expected = BitArray9::from_indices(&[2, 3, 4, 5, 6]);
        assert_eq!(array1 | array2, expected);
    }

    #[test]
    fn test_bitxor() {
        let array1 = BitArray9::from_indices(&[1, 3]);
        let array2 = BitArray9::from_indices(&[2, 3]);
        let expected = BitArray9::from_indices(&[1, 2]);
        assert_eq!(array1 ^ array2, expected);
    }

    #[test]
    fn test_bitand() {
        let array1 = BitArray9::from_indices(&[1, 3]);
        let array2 = BitArray9::from_indices(&[2, 3]);
        let expected = BitArray9::from_indices(&[3]);
        assert_eq!(array1 & array2, expected);
    }
}
