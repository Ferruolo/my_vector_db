use crate::vector_b_tree::BTree;

mod vector_b_tree;


fn main() {
    let mut tree = BTree::new();
    let strings: Vec<String> = vec![
        String::from("E"),
        String::from("F"),
        String::from("T"),
        String::from("Q")
    ];

    println!("Adding 9");
    tree.set_item(9, strings[0].clone());
    println!("Adding 10");
    tree.set_item(10, strings[1].clone());
    println!("Adding 12");
    tree.set_item(12, strings[2].clone());
    println!("Adding 23");
    tree.set_item(23, strings[3].clone());

    tree.print();
}