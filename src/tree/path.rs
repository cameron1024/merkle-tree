use core::iter::zip;

use halo2_proofs::pasta::pallas;

use crate::{circuits::insert::hash_merge, Element, Tree};

use super::repr::Node;

pub struct Path<const N: usize> {
    /// The siblings that form the hash
    pub(crate) siblings: Vec<Element>,
}

impl<const N: usize> Tree<N> {
    pub fn path_for(&self, element: Element) -> Path<N> {
        // N = 1 means depth = 1 (i.e. single element)
        let bits = element.least_significant_bits(N - 1);
        let mut current_node = &self.tree_repr;
        let mut siblings = Vec::with_capacity(N);

        for is_right in bits {
            match current_node {
                Node::Nulls(0) | Node::Leaf(_) => break,
                Node::Nulls(n) => {
                    for i in (0..*n).rev() {
                        siblings.push(Node::Nulls(i).hash());
                    }

                    break;
                }
                Node::Parent { left, right, .. } => match is_right {
                    true => {
                        siblings.push(left.hash());
                        current_node = right;
                    }
                    false => {
                        siblings.push(right.hash());
                        current_node = left;
                    }
                },
            }
        }

        siblings.reverse();

        Path { siblings }
    }
}

impl<const N: usize> Path<N> {
    pub fn compute_root(&self, element: Element) -> Element {
        let bits = element.least_significant_bits(N);

        // begin computing the root
        let mut root = element;

        for (bit, &sibling) in zip(bits.into_iter().rev(), &self.siblings) {
            match bit {
                // bit is 0, we are on the left
                false => root = hash_merge(root, sibling),
                // bit is 1, we are on the right
                true => root = hash_merge(sibling, root),
            }
        }

        root
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_test() {
        let mut tree = Tree::<64>::new();

        for i in 0..10 {
            tree.insert(Element::from_u64(i));
        }

        for i in 0..10 {
            let e = Element::from_u64(i);
            let path = tree.path_for(e);
            assert_eq!(path.compute_root(e), tree.root_hash());
        }

        for i in 10..20 {
            let e = Element::from_u64(i);
            let path = tree.path_for(e);
            assert_eq!(path.compute_root(Element::NULL), tree.root_hash());
        }
    }
}
