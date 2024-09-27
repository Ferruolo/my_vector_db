use rand::seq::SliceRandom;
use crate::vector_b_tree::BTree;
mod vector_b_tree;

const ELEMENTS_PER_PAGE: usize = 4;

fn main () {
    let mut tree = BTree::new();
    let mut rng = rand::thread_rng();
    let mut indices: Vec<usize> = (0..1000).collect();
    indices.shuffle(&mut rng);
    for i in 0..1000 {
        tree.set_item(i, i.to_string());
    }


    for &i in &indices {
        tree.remove(i);
        assert_eq!(tree.get_item(i), None);
    }

    assert_eq!(tree.get_num_elements(), 0);
}

