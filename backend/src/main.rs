use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng;
use crate::vector_b_tree::BTree;
mod vector_b_tree;

const ELEMENTS_PER_PAGE: usize = 4;

fn main() {
    let mut tree = BTree::new();
    for i in 0..10000 {
        tree.set_item(i, i.to_string());
    }
    println!("Max Depth {}", tree.max_depth);
    // tree.print();
    assert!(tree.max_depth > 3);
}