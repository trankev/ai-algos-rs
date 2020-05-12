use super::MaskComparison;
use generic_array;
use num;
use num::traits::One;
use num::traits::Zero;
use std::mem;
use typenum::Unsigned;

pub trait BitArraySettings {
    const SIZE: usize;
    type FirstBitType: num::PrimInt + Default;
    const INTEGER_SIZE: usize = 8 * mem::size_of::<Self::FirstBitType>();
    type ArrayLength: generic_array::ArrayLength<Self::FirstBitType>;
    type LastBitType: num::PrimInt + Default;
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GenericBitArray<Settings: BitArraySettings> {
    first_bits: generic_array::GenericArray<Settings::FirstBitType, Settings::ArrayLength>,
    last_bits: Settings::LastBitType,
}

impl<Settings: BitArraySettings> GenericBitArray<Settings> {
    fn zero() -> GenericBitArray<Settings> {
        GenericBitArray {
            first_bits: generic_array::GenericArray::default(),
            last_bits: Settings::LastBitType::default(),
        }
    }

    fn from_indices(indices: &[usize]) -> Self {
        let mut result = Self::zero();
        for index in indices {
            result.set(*index);
        }
        result
    }

    fn isset(&self, slot: usize) -> bool {
        debug_assert!(
            slot < Settings::SIZE,
            format!("BitArray slot out of bound: {} >= {}", slot, Settings::SIZE),
        );
        let index = slot / Settings::INTEGER_SIZE;
        let offset = slot % Settings::INTEGER_SIZE;
        println!("{} {}", index, offset);
        if index < Settings::ArrayLength::to_usize() {
            let mask = Settings::FirstBitType::one() << offset;
            self.first_bits[index] & mask == mask
        } else {
            let mask = Settings::LastBitType::one() << offset;
            self.last_bits & mask == mask
        }
    }

    fn set(&mut self, slot: usize) {
        debug_assert!(
            slot < Settings::SIZE,
            format!("BitArray slot out of bound: {} >= {}", slot, Settings::SIZE),
        );
        let index = slot / Settings::INTEGER_SIZE;
        let offset = slot % Settings::INTEGER_SIZE;
        println!("{} {}", index, offset);
        if index < Settings::ArrayLength::to_usize() {
            let mask = Settings::FirstBitType::one() << offset;
            self.first_bits[index] = self.first_bits[index] | mask;
        } else {
            let mask = Settings::LastBitType::one() << offset;
            self.last_bits = self.last_bits | mask;
        }
    }

    fn swap(&self, permutation: &[usize]) -> Self {
        let mut result = Self::zero();
        let mut iterator = permutation.iter();
        for bitset in &self.first_bits {
            let mut mask = Settings::FirstBitType::one();
            for _ in 0..Settings::INTEGER_SIZE {
                match iterator.next() {
                    Some(permuted) => {
                        if *bitset & mask == mask {
                            result.set(*permuted);
                        }
                    }
                    None => unreachable!(),
                }
                mask = mask << 1;
            }
        }
        let mut mask = Settings::LastBitType::one();
        for permuted in iterator {
            if self.last_bits & mask == mask {
                result.set(*permuted);
            }
            mask = mask << 1;
        }
        result
    }

    fn compare_with_mask(&self, mask: &GenericBitArray<Settings>) -> MaskComparison {
        let mut is_zero = true;
        let mut is_equal = true;
        for index in 0..Settings::ArrayLength::to_usize() {
            let masked = self.first_bits[index] & mask.first_bits[index];
            if masked != mask.first_bits[index] {
                is_equal = false;
            }
            if masked != Settings::FirstBitType::zero() {
                is_zero = false;
            }
        }
        let masked = self.last_bits & mask.last_bits;
        if masked != mask.last_bits {
            is_equal = false;
        }
        if masked != Settings::LastBitType::zero() {
            is_zero = false;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct BitArray225Settings {}
    impl BitArraySettings for BitArray225Settings {
        const SIZE: usize = 225;
        type FirstBitType = u64;
        type ArrayLength = typenum::U3;
        type LastBitType = u64;
    }

    type BitArray225 = GenericBitArray<BitArray225Settings>;

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
        println!("{:?}", instance);
        for index in 0..225 {
            println!("{}", index);
            if indices.contains(&index) {
                assert!(instance.isset(index));
            } else {
                assert!(!instance.isset(index));
            }
        }
    }
}
