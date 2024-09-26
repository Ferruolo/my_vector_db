use crate::vector_b_tree::BTree;

mod vector_b_tree;

fn main () {
    let mut tree = BTree::new();
    for i in 0..1000 {
        tree.set_item(i, format!("Original {}", i));
    }
    
    
    
    for i in 0..1000 {
        tree.set_item(i, format!("Updated {}", i));
        tree.print();
        assert_eq!(tree.get_num_elements(), 1000);
    }


    for i in 0..1000 {
        assert_eq!(tree.get_item(i), Some(&format!("Updated {}", i)));
    }
}