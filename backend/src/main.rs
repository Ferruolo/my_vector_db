use crate::vector_b_tree::BTree;

mod vector_b_tree;

fn main () {
    let mut tree = BTree::new();
    
    tree.print();
}