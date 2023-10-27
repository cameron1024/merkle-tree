use std::collections::hash_map::Values;

use bitvec::vec::BitVec;

use crate::{Element, Tree};

#[derive(Debug, Clone)]
pub struct Elements<'a, const N: usize> {
    elements: Values<'a, BitVec<u64>, Element>,
}

impl<'a, const N: usize> Iterator for Elements<'a, N> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.next().copied()
    }
}

impl<const N: usize> Tree<N> {
    pub fn elements(&self) -> Elements<N> {
        Elements {
            elements: self.nodes.values(),
        }
    }
}
