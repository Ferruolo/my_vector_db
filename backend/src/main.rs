use crate::b_tree_vanilla::BTree;
mod b_tree_vanilla;
mod vector_db;
mod llama_embedding;

fn main() {
    let mut tree = BTree::new();
    let strings: Vec<String> = vec![
        String::from("E"),
        String::from("G"),
        String::from("T"),
        String::from("Q"),
        String::from("F"),
        String::from("B"),
        String::from("A"),
        String::from("A"),
        String::from("F"),
        String::from("V"),
        String::from("V"),
        String::from("H"),
        String::from("L"),
        String::from("Alpha"),
        String::from("Omega"),
    ];

    tree.set_item(9, strings[0].clone());
    tree.set_item(10, strings[1].clone());
    tree.set_item(12, strings[2].clone());
    tree.set_item(23, strings[3].clone());
    tree.set_item(5, strings[4].clone());
    // tree.set_item(2, strings[5].clone());
    // tree.set_item(7, strings[6].clone());
    // tree.set_item(38, strings[7].clone());
    // tree.set_item(39, strings[8].clone());
    // tree.set_item(40, strings[9].clone());
    // tree.set_item(45, strings[10].clone());
    //
    // tree.set_item(0, strings[11].clone());
    // tree.set_item(1, strings[12].clone());
    // tree.set_item(50, strings[13].clone());
    // tree.set_item(55, strings[14].clone());
    //
    // tree.print();
    // tree.remove(5);
    // tree.remove(9);
    // tree.print();
    // tree.remove(10);
    // tree.remove(7);
    // tree.print();
    // tree.set_item(22, "A".to_string());
    // tree.remove(12);
    // tree.print();
    // tree.remove(2);
    tree.print();
}