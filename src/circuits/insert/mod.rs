//! This module exposes a circuit that proves the following:
//!
//! Given a sparse merkle tree with root hash `old_root_hash`, after inserting the key `hash`, the
//! root hash becomes `new_root_hash.
//! Additionally, prior to the insert, `hash` was not present in the tree.
//!
//! `old_root_hash`, `new_root_hash`, and `hash` are public inputs.
//! The full contents of the tree remains private.
//!
//! To generate the proof, we do the following:
//!  - we can prove the existence of a particular key at a particular location by generating a path
//!  to that node, and calculating the hashes and verifying that the final hash is equal to the
//!  root hast of the tree.
//!  - given this, we can prove that the key does *not* exist by generating a merkle path and
//!  verifying that it verifies with the null hash value
//!  - then, we can prove that

use color_eyre::Result;
use halo2_gadgets::poseidon::primitives::Hash;
use rand_chacha::rand_core::OsRng;

use crate::{Element, Tree};

type Base = halo2_proofs::pasta::pallas::Base;

mod chip;
mod circuit;
mod config;
mod proof;

pub use proof::Proof;

pub fn prove_insert<const N: usize>(tree: &Tree<N>, new_hash: Element) -> Proof {
    proof::create(&mut OsRng, tree, new_hash)
}

/// Merge two hashes
///
/// This is the function that computes the hash of a parent node
pub fn hash_merge(left: Element, right: Element) -> Element {
    let hash = Hash::<_, chip::PoseidonSettings, _, 3, 2>::init().hash([left.0, right.0]);
    Element(hash)
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;

    use super::*;

    #[test]
    fn simple_test() {
        let tree = Tree::<64>::new();
        let proof = prove_insert(&tree, Element::from_u64(123));
    }
}
