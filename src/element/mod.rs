use std::hash::{Hash, Hasher};

use bitvec::vec::BitVec;
use halo2_proofs::pasta::{group::ff::PrimeFieldBits, pallas};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Element(pub(crate) pallas::Base);

impl Hash for Element {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_le_bits().hash(state);
    }
}

impl Element {
    /// The canonical null hash, representing an absence of a value
    ///
    /// This value is not special, it was chosen arbitrarily, since the probability of finding a
    /// cleartext that hashes to any specific value is ~0.
    pub const NULL: Self = Self(pallas::Base::from_raw([0; 4]));

    pub fn from_u64(elem: u64) -> Self {
        Self(pallas::Base::from(elem))
    }

    /// Get the last `count` bits of this hash, with most significant bit first
    pub(crate) fn least_significant_bits(&self, count: usize) -> BitVec<u64> {
        let bits = self.0.to_le_bits();
        let mut vec = bits[0..count].to_bitvec();
        vec.reverse();
        vec
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;

    use super::*;

    #[test]
    fn element_lsb_test() {
        fn bits(n: u64, bits: usize) -> BitVec<u64> {
            Element::from_u64(n).least_significant_bits(bits)
        }

        assert_eq!(bits(0, 1), bitvec![0]);
        assert_eq!(bits(1, 1), bitvec![1]);

        assert_eq!(bits(0, 4), bitvec![0, 0, 0, 0]);
        assert_eq!(bits(1, 4), bitvec![0, 0, 0, 1]);
        assert_eq!(bits(3, 4), bitvec![0, 0, 1, 1]);

        assert_eq!(bits(0, 6), bitvec![0, 0, 0, 0, 0, 0]);
        assert_eq!(bits(1, 6), bitvec![0, 0, 0, 0, 0, 1]);
        assert_eq!(bits(3, 6), bitvec![0, 0, 0, 0, 1, 1]);
    }
}
