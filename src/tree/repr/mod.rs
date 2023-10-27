use std::sync::OnceLock;

use bitvec::slice::BitSlice;

use crate::{circuits::insert::hash_merge, Element};

/// Get the precomputed list of null hashes (`null`, `hash(null, null)`, `hash(hash(null, null),
/// hash(null, null))`, etc.)
pub(super) fn hash_table() -> &'static [Element] {
    static HASH_TABLE: OnceLock<Vec<Element>> = OnceLock::new();

    HASH_TABLE.get_or_init(|| {
        let max_hashes = 128;

        let mut result = Vec::with_capacity(max_hashes);
        let mut hash = Element::NULL;

        for _ in 0..max_hashes {
            result.push(hash);
            hash = hash_merge(hash, hash);
        }

        result
    })
}

#[test]
fn first_hash_is_null() {
    use halo2_proofs::{arithmetic::Field, pasta::pallas};

    assert_eq!(hash_table()[0], Element(pallas::Base::ZERO));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Node {
    Leaf(Element),
    Nulls(usize),
    Parent {
        left: Box<Node>,
        right: Box<Node>,
        hash: Element,
    },
}

impl Node {
    /// Create a new, empty tree
    ///
    /// A depth of 0 corresponds to a single null hash
    pub fn new(depth: usize) -> Self {
        Self::Nulls(depth - 1)
    }

    /// The hash of this node
    pub fn hash(&self) -> Element {
        match self {
            Self::Leaf(hash) => *hash,
            Self::Parent { hash, .. } => *hash,
            Self::Nulls(i) => hash_table()[*i],
        }
    }

    pub fn insert(&mut self, bits: &BitSlice<u64>, element: Element) {
        match self {
            Self::Leaf(node) => panic!("trying to insert into occupied node: {node:?}"),
            Self::Parent { left, right, hash } => {
                let (head, tail) = bits.split_first().unwrap();
                match *head {
                    false => left.insert(tail, element),
                    true => right.insert(tail, element),
                };

                *hash = hash_merge(left.hash(), right.hash());
            }
            // We have reached the final null
            Self::Nulls(0) => *self = Self::Leaf(element),
            Self::Nulls(depth) => {
                // otherwise, we split the aggregated nulls into a regular parent

                *self = Self::Parent {
                    left: Box::new(Self::Nulls(*depth - 1)),
                    right: Box::new(Self::Nulls(*depth - 1)),
                    hash: hash_table()[*depth],
                };

                // now try again
                self.insert(bits, element);
            }
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            Self::Leaf(_) => 1,
            Self::Nulls(i) => i + 1,
            #[cfg(not(debug_assertions))]
            Self::Parent { left, .. } => left.depth() + 1,
            #[cfg(debug_assertions)]
            Self::Parent { left, right, .. } => {
                let left_depth = left.depth();
                let right_depth = right.depth();
                assert_eq!(left_depth, right_depth);
                left_depth + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::pasta::pallas;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn depth_test() {
        for i in 1..128 {
            let tree = Node::new(i);
            assert_eq!(tree.depth(), i);
        }
    }

    #[test]
    fn root_hash_makes_sense() {
        for i in 1..100 {
            assert_eq!(Node::new(i).hash(), hash_table()[i - 1]);
        }
    }

    #[test]
    fn simple_manual_insert() {
        let mut tree = Node::new(3);

        let element = Element(pallas::Base::from(1u64));
        let bits = element.least_significant_bits(2);

        tree.insert(&bits, element);

        let ll = Element::NULL;
        let lr = Element(pallas::Base::from(1u64));
        let rl = Element::NULL;
        let rr = Element::NULL;

        let l = hash_merge(ll, lr);
        let r = hash_merge(rl, rr);

        let expected_root = hash_merge(l, r);

        let expected_tree = Node::Parent {
            left: Box::new(Node::Parent {
                left: Box::new(Node::Nulls(0)),
                right: Box::new(Node::Leaf(element)),
                hash: l,
            }),
            right: Box::new(Node::Nulls(1)),
            hash: expected_root,
        };

        assert_eq!(tree, expected_tree);

        assert_eq!(tree.hash(), expected_root);
    }
}
