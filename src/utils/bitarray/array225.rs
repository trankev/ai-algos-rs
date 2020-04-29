use super::BitArray;
use auto_ops::*;
use std::mem;

const BIT_COUNT: u8 = 225;
type IntegerType = u64;
const INTEGER_SIZE: u8 = 8 * mem::size_of::<IntegerType>() as u8;
const ARRAY_SIZE: usize = (BIT_COUNT / INTEGER_SIZE) as usize + 1;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitArray225 {
    bits: [u64; ARRAY_SIZE],
}

impl BitArray for BitArray225 {
    type Index = u8;
    fn zero() -> BitArray225 {
        BitArray225 {
            bits: [0; ARRAY_SIZE],
        }
    }

    fn from_indices(indices: &[Self::Index]) -> Self {
        let mut result = BitArray225::zero();
        for index in indices {
            result.set(*index);
        }
        result
    }

    fn isset(&self, index: Self::Index) -> bool {
        debug_assert!(
            index < BIT_COUNT,
            format!("BitArray index out of bound: {} >= {}", index, BIT_COUNT)
        );
        let integer = index / INTEGER_SIZE;
        let offset = index % INTEGER_SIZE;
        let mask = 1 << offset;
        self.bits[integer as usize] & mask == mask
    }

    fn set(&mut self, index: Self::Index) {
        let integer = index / INTEGER_SIZE;
        let offset = index % INTEGER_SIZE;
        self.bits[integer as usize] |= 1 << offset;
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
