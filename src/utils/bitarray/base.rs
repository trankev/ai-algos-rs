use super::comparison;
use super::settings;
use num::traits::One;
use num::traits::PrimInt;
use num::traits::Zero;
use std::ops;
use typenum::Unsigned;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitArray<Settings: settings::BitArraySettings> {
    first_bits: generic_array::GenericArray<Settings::FirstBitType, Settings::ArrayLength>,
    last_bits: Settings::LastBitType,
}

impl<Settings: settings::BitArraySettings> BitArray<Settings> {
    pub fn zero() -> BitArray<Settings> {
        BitArray {
            first_bits: generic_array::GenericArray::default(),
            last_bits: Settings::LastBitType::default(),
        }
    }

    pub fn from_indices(indices: &[usize]) -> Self {
        let mut result = Self::zero();
        for index in indices {
            result.set(*index);
        }
        result
    }

    pub fn isset(&self, slot: usize) -> bool {
        debug_assert!(
            slot < Settings::SIZE,
            format!("BitArray slot out of bound: {} >= {}", slot, Settings::SIZE),
        );
        let index = slot / Settings::INTEGER_SIZE;
        let offset = slot % Settings::INTEGER_SIZE;
        if index < Settings::ArrayLength::to_usize() {
            let mask = Settings::FirstBitType::one() << offset;
            self.first_bits[index] & mask == mask
        } else {
            let mask = Settings::LastBitType::one() << offset;
            self.last_bits & mask == mask
        }
    }

    pub fn set(&mut self, slot: usize) {
        debug_assert!(
            slot < Settings::SIZE,
            format!("BitArray slot out of bound: {} >= {}", slot, Settings::SIZE),
        );
        let index = slot / Settings::INTEGER_SIZE;
        let offset = slot % Settings::INTEGER_SIZE;
        if index < Settings::ArrayLength::to_usize() {
            let mask = Settings::FirstBitType::one() << offset;
            self.first_bits[index] = self.first_bits[index] | mask;
        } else {
            let mask = Settings::LastBitType::one() << offset;
            self.last_bits = self.last_bits | mask;
        }
    }

    pub fn unset(&mut self, slot: usize) {
        debug_assert!(
            slot < Settings::SIZE,
            format!("BitArray slot out of bound: {} >= {}", slot, Settings::SIZE),
        );
        let index = slot / Settings::INTEGER_SIZE;
        let offset = slot % Settings::INTEGER_SIZE;
        if index < Settings::ArrayLength::to_usize() {
            let mask = !(Settings::FirstBitType::one() << offset);
            self.first_bits[index] = self.first_bits[index] & mask;
        } else {
            let mask = !(Settings::LastBitType::one() << offset);
            self.last_bits = self.last_bits & mask;
        }
    }

    pub fn swap(&self, permutation: &[usize]) -> Self {
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

    pub fn compare_with_mask(&self, mask: &BitArray<Settings>) -> comparison::MaskComparison {
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
            comparison::MaskComparison::Equal
        } else if is_zero {
            comparison::MaskComparison::Zero
        } else {
            comparison::MaskComparison::Partial
        }
    }

    pub fn count_ones(&self) -> u32 {
        let mut result = self.first_bits.iter().map(|x| x.count_ones()).sum();
        result += self.last_bits.count_ones();
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitAnd<BitArray<Settings>> for BitArray<Settings> {
    type Output = BitArray<Settings>;
    fn bitand(self, rhs: BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] & rhs.first_bits[index];
        }
        result.last_bits = self.last_bits & rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitAnd<&BitArray<Settings>> for BitArray<Settings> {
    type Output = BitArray<Settings>;
    fn bitand(self, rhs: &BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] & rhs.first_bits[index];
        }
        result.last_bits = self.last_bits & rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitAnd<BitArray<Settings>> for &BitArray<Settings> {
    type Output = BitArray<Settings>;
    fn bitand(self, rhs: BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] & rhs.first_bits[index];
        }
        result.last_bits = self.last_bits & rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitAnd<&BitArray<Settings>>
    for &BitArray<Settings>
{
    type Output = BitArray<Settings>;
    fn bitand(self, rhs: &BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] & rhs.first_bits[index];
        }
        result.last_bits = self.last_bits & rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitOr<BitArray<Settings>> for BitArray<Settings> {
    type Output = BitArray<Settings>;
    fn bitor(self, rhs: BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] | rhs.first_bits[index];
        }
        result.last_bits = self.last_bits | rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitOr<&BitArray<Settings>> for BitArray<Settings> {
    type Output = BitArray<Settings>;
    fn bitor(self, rhs: &BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] | rhs.first_bits[index];
        }
        result.last_bits = self.last_bits | rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitOr<BitArray<Settings>> for &BitArray<Settings> {
    type Output = BitArray<Settings>;
    fn bitor(self, rhs: BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] | rhs.first_bits[index];
        }
        result.last_bits = self.last_bits | rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitOr<&BitArray<Settings>> for &BitArray<Settings> {
    type Output = BitArray<Settings>;
    fn bitor(self, rhs: &BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] | rhs.first_bits[index];
        }
        result.last_bits = self.last_bits | rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitXor<BitArray<Settings>> for BitArray<Settings> {
    type Output = BitArray<Settings>;
    fn bitxor(self, rhs: BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] ^ rhs.first_bits[index];
        }
        result.last_bits = self.last_bits ^ rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitXor<&BitArray<Settings>> for BitArray<Settings> {
    type Output = BitArray<Settings>;
    fn bitxor(self, rhs: &BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] ^ rhs.first_bits[index];
        }
        result.last_bits = self.last_bits ^ rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitXor<BitArray<Settings>> for &BitArray<Settings> {
    type Output = BitArray<Settings>;
    fn bitxor(self, rhs: BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] ^ rhs.first_bits[index];
        }
        result.last_bits = self.last_bits ^ rhs.last_bits;
        result
    }
}

impl<Settings: settings::BitArraySettings> ops::BitXor<&BitArray<Settings>>
    for &BitArray<Settings>
{
    type Output = BitArray<Settings>;
    fn bitxor(self, rhs: &BitArray<Settings>) -> Self::Output {
        let mut result = BitArray::zero();
        for index in 0..Settings::ArrayLength::to_usize() {
            result.first_bits[index] = self.first_bits[index] ^ rhs.first_bits[index];
        }
        result.last_bits = self.last_bits ^ rhs.last_bits;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
    struct BitArray225Settings {}
    impl settings::BitArraySettings for BitArray225Settings {
        const SIZE: usize = 225;
        type FirstBitType = u64;
        type ArrayLength = typenum::U3;
        type LastBitType = u64;
    }

    type BitArray225 = BitArray<BitArray225Settings>;

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
    fn test_unset() {
        let mut instance = BitArray225::from_indices(&[6, 10, 100, 120, 220, 222]);
        for index in &[10, 100, 220] {
            assert!(instance.isset(*index));
            instance.unset(*index);
            assert!(!instance.isset(*index));
        }
        for index in &[6, 120, 222] {
            assert!(instance.isset(*index));
        }
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
}
