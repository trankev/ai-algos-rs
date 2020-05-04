use super::{BitArray, MaskComparison};
use auto_ops::*;
use std::mem;

const BIT_COUNT: usize = 225;
type IntegerType = u64;
const INTEGER_SIZE: usize = 8 * mem::size_of::<IntegerType>();
const ARRAY_SIZE: usize = (BIT_COUNT / INTEGER_SIZE) as usize + 1;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitArray225 {
    bits: [u64; ARRAY_SIZE],
}

impl BitArray for BitArray225 {
    fn zero() -> BitArray225 {
        BitArray225 {
            bits: [0; ARRAY_SIZE],
        }
    }

    fn isset(&self, index: usize) -> bool {
        debug_assert!(
            index < BIT_COUNT,
            format!("BitArray index out of bound: {} >= {}", index, BIT_COUNT)
        );
        let integer = index / INTEGER_SIZE;
        let offset = index % INTEGER_SIZE;
        let mask = 1 << offset;
        self.bits[integer as usize] & mask == mask
    }

    fn set(&mut self, index: usize) {
        debug_assert!(
            index < BIT_COUNT,
            format!("BitArray index out of bound: {} >= {}", index, BIT_COUNT)
        );
        let integer = index / INTEGER_SIZE;
        let offset = index % INTEGER_SIZE;
        self.bits[integer as usize] |= 1 << offset;
    }

    fn swap(&self, permutation: &[usize]) -> Self {
        let mut result = Self::zero();
        let mut current_bit = self.bits[0];
        let mut bit_index = 0;
        let mut mask = 1;
        for (index, permuted) in permutation.iter().enumerate() {
            if current_bit & mask == mask {
                result.set(*permuted);
            }
            if index % 64 == 63 {
                mask = 1;
                bit_index += 1;
                current_bit = self.bits[bit_index];
            } else {
                mask <<= 1;
            }
        }
        result
    }

    fn compare_with_mask(&self, mask: &BitArray225) -> MaskComparison {
        let mut is_zero = true;
        let mut is_equal = true;
        for index in 0..ARRAY_SIZE {
            let masked = self.bits[index] & mask.bits[index];
            if masked != mask.bits[index] {
                is_equal = false;
            }
            if masked != 0 {
                is_zero = false;
            }
        }
        if is_equal {
            MaskComparison::Equal
        } else if is_zero {
            MaskComparison::Zero
        } else {
            MaskComparison::Partial
        }
    }
}

macro_rules! binary_bit_op {
    ($op:tt) => {
        impl_op_ex!($op |a: &BitArray225, b: &BitArray225| -> BitArray225 {
            let mut result = BitArray225 {
                bits: [0; ARRAY_SIZE],
            };
            for index in 0..ARRAY_SIZE {
                result.bits[index] = a.bits[index] $op b.bits[index];
            }
            result
        });
    }
}

binary_bit_op!(&);
binary_bit_op!(|);
binary_bit_op!(^);

#[cfg(test)]
mod tests {
    use super::super::BitArray;
    use super::*;

    #[test]
    fn test_zero() {
        let instance = BitArray225::zero();
        for index in 0..225 {
            assert!(!instance.isset(index));
        }
    }

    #[test]
    #[should_panic]
    fn test_out_of_bound() {
        let instance = BitArray225::zero();
        instance.isset(250);
    }

    #[test]
    fn test_from_positions() {
        let indices = vec![20, 200, 77];
        let instance = BitArray225::from_indices(&indices);
        for index in 0..225 {
            if indices.contains(&index) {
                assert!(instance.isset(index));
            } else {
                assert!(!instance.isset(index));
            }
        }
    }

    #[test]
    fn test_bitor() {
        let array1 = BitArray225::from_indices(&[202, 103, 5]);
        let array2 = BitArray225::from_indices(&[202, 104, 6]);
        let expected = BitArray225::from_indices(&[202, 103, 104, 5, 6]);
        assert_eq!(&array1 | &array2, expected);
    }

    #[test]
    fn test_bitxor() {
        let array1 = BitArray225::from_indices(&[101, 203]);
        let array2 = BitArray225::from_indices(&[102, 203]);
        let expected = BitArray225::from_indices(&[101, 102]);
        assert_eq!(&array1 ^ &array2, expected);
    }

    #[test]
    fn test_bitand() {
        let array1 = BitArray225::from_indices(&[101, 203]);
        let array2 = BitArray225::from_indices(&[102, 203]);
        let expected = BitArray225::from_indices(&[203]);
        assert_eq!(&array1 & &array2, expected);
    }
}
