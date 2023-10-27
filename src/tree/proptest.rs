use halo2_proofs::pasta::{group::ff::FromUniformBytes, pallas};
use proptest::{arbitrary::StrategyFor, prelude::*, strategy::Map};

use crate::Element;

type U512 = [u8; 64];

impl Arbitrary for Element {
    type Strategy = Map<StrategyFor<U512>, fn(U512) -> Self>;
    type Parameters = ();

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        any::<U512>().prop_map(|bytes| Element(pallas::Base::from_uniform_bytes(&bytes)))
    }
}
