use std::collections::HashMap;

use bitvec::vec::BitVec;

use crate::Element;

use self::repr::Node;

mod insert_and_prove;
mod iter;
mod path;
mod repr;

#[cfg(any(test, feature = "proptest"))]
mod proptest;

#[cfg(test)]
mod tests;

/// A sparse merkle tree with configurable depth
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<const SIZE: usize = 64> {
    /// A cached version of the tree that makes computing the root hash easier
    tree_repr: Node,
    nodes: HashMap<BitVec<u64>, Element>,
}

impl<const N: usize> Default for Tree<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Tree<N> {
    /// Create a new, empty tree
    pub fn new() -> Self {
        assert_ne!(N, 0, "a tree depth of 0 makes no sense");

        Self {
            tree_repr: Node::new(N),
            nodes: HashMap::new(),
        }
    }

    /// The number of elements in the tree
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` if the tree contains no elements
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// The root hash of this tree
    pub fn root_hash(&self) -> Element {
        self.tree_repr.hash()
    }

    pub fn contains(&self, element: Element) -> bool {
        let bits = Self::bits_for_element(element);

        self.nodes.contains_key(&bits)
    }

    /// Insert an element into the tree
    pub fn insert(&mut self, element: Element) {
        let bits = Self::bits_for_element(element);

        self.tree_repr.insert(&bits, element);
        self.nodes.insert(bits, element);
    }

    fn bits_for_element(element: Element) -> BitVec<u64> {
        element.least_significant_bits(N - 1)
    }
}
