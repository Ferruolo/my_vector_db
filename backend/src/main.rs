use crate::vector_b_tree::BTree;
mod vector_b_tree;


fn main() {
    let mut tree = BTree::new();
    let strings: Vec<String> = vec![
        String::from("E"),
        String::from("G"),
        String::from("T"),
        String::from("Q"),
        String::from("F")
    ];


    tree.set_item(9, strings[0].clone());
    tree.set_item(10, strings[1].clone());
    tree.set_item(12, strings[2].clone());
    tree.set_item(23, strings[3].clone());
    tree.set_item(5, strings[4].clone());
    // assert_eq!(tree.get_num_elements(), 4);

    tree.print();
    tree.remove(5);
    tree.print();
}