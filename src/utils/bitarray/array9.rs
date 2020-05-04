use super::{BitArray, MaskComparison};
use auto_ops::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitArray9 {
    bits: u16,
}

impl BitArray for BitArray9 {
    fn zero() -> BitArray9 {
        BitArray9 { bits: 0 }
    }

    fn isset(&self, index: usize) -> bool {
        debug_assert!(
            index < 9,
            format!("BitArray index out of bound: {} >= 9", index)
        );
        let mask = 1u16 << index;
        self.bits & mask == mask
    }

    fn set(&mut self, index: usize) {
        self.bits |= 1u16 << index;
    }

    fn compare_with_mask(&self, mask: &BitArray9) -> MaskComparison {
        let masked = self.bits & mask.bits;
        if masked == mask.bits {
            MaskComparison::Equal
        } else if masked == 0 {
            MaskComparison::Zero
        } else {
            MaskComparison::Partial
        }
    }
}

impl_op_ex!(&|a: &BitArray9, b: &BitArray9| -> BitArray9 {
    BitArray9 {
        bits: a.bits & b.bits,
    }
});

impl_op_ex!(| |a: &BitArray9, b: &BitArray9| -> BitArray9 {
    BitArray9 {
        bits: a.bits | b.bits,
    }
});

impl_op_ex!(^ |a: &BitArray9, b: &BitArray9| -> BitArray9 {
    BitArray9 {
        bits: a.bits ^ b.bits,
    }
});

#[cfg(test)]
mod tests {
    use super::super::BitArray;
    use super::BitArray9;

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
        assert_eq!(&array1 | &array2, expected);
    }

    #[test]
    fn test_bitxor() {
        let array1 = BitArray9::from_indices(&[1, 3]);
        let array2 = BitArray9::from_indices(&[2, 3]);
        let expected = BitArray9::from_indices(&[1, 2]);
        assert_eq!(&array1 ^ &array2, expected);
    }

    #[test]
    fn test_bitand() {
        let array1 = BitArray9::from_indices(&[1, 3]);
        let array2 = BitArray9::from_indices(&[2, 3]);
        let expected = BitArray9::from_indices(&[3]);
        assert_eq!(&array1 & &array2, expected);
    }

    #[test]
    fn test_swap() {
        let array = BitArray9::from_indices(&[0, 1, 2]);
        let result = array.swap(&[8, 5, 2, 7, 4, 1, 6, 3, 0]);
        let expected = BitArray9::from_indices(&[2, 5, 8]);
        assert_eq!(result, expected);
    }
}
