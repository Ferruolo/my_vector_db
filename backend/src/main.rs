use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng;
use crate::vector_b_tree::BTree;
mod vector_b_tree;

const ELEMENTS_PER_PAGE: usize = 4;

fn main() {
    let mut tree = BTree::new();
    let seed: u64 = rand::thread_rng().gen();
    println!("Using seed: {}", seed);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut indices: Vec<usize> = (0..1000000).collect();
    indices.shuffle(&mut rng);
    for i in 0..1000000 {
        tree.set_item(i, i.to_string());
    }
    indices.shuffle(&mut rng);
    for (count, &i) in indices.iter().enumerate() {
        tree.remove(i);
        assert_eq!(tree.get_item(i), None);
    }

    assert_eq!(tree.get_num_elements(), 0);
}