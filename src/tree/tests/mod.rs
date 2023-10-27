use std::collections::HashSet;

use test_strategy::proptest;

use crate::{Element, Tree};

#[proptest]
fn can_insert(elements: HashSet<Element>) {
    let mut tree = Tree::<64>::new();

    for &element in &elements {
        tree.insert(element);
    }

    assert_eq!(elements, tree.elements().collect::<HashSet<_>>());

    for element in elements {
        assert!(tree.contains(element));
    }
}
