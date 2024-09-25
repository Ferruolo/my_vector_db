use std::error::Error;
use crate::vector_b_tree::BTree;

mod vector_b_tree;

fn main () {
    let mut tree = BTree::new();
    tree.insert(0, "Zero".to_string());
    tree.insert(9, "Nine".to_string());
    tree.insert(1, "1".to_string());
    tree.insert(4, "4".to_string());
    
    tree.insert(1, "1".to_string());
    tree.insert(5, "Five".to_string());
    
    
    tree.print();
    assert_eq!(tree.get_item(0), Some("Zero".to_string()));
}