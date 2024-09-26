use std::error::Error;
use crate::vector_b_tree::BTree;

mod vector_b_tree;

fn main () {
    let mut tree = BTree::new();
    tree.insert(0, "Zero".to_string());
    tree.insert(9, "Nine".to_string());
    tree.insert(1, "1".to_string());
    tree.insert(4, "4".to_string());
    
    tree.insert(5, "Five".to_string());
    tree.insert(16, "16".to_string());
    tree.insert(7, "7".to_string());
    tree.insert(19, "19".to_string());

    tree.insert(13, "13".to_string());

    tree.print();

}