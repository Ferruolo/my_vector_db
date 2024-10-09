use crate::vector_b_tree::BTree;
mod vector_b_tree;


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
    tree.set_item(2, strings[5].clone());
    tree.set_item(7, strings[6].clone());
    tree.set_item(38, strings[7].clone());
    tree.set_item(39, strings[8].clone());
    tree.set_item(40, strings[9].clone());
    tree.set_item(45, strings[10].clone());
    tree.print();
    tree.set_item(0, strings[11].clone());
    tree.set_item(1, strings[12].clone());
    tree.set_item(50, strings[13].clone());
    tree.set_item(55, strings[14].clone());

    println!("Tree depth: {}", tree.get_depth());
    println!("Item at key 9: {:?}", tree.get_item(9));
    println!("Item at key 10: {:?}", tree.get_item(10));
    println!("Item at key 11: {:?}", tree.get_item(11));
    println!("Item at key 12: {:?}", tree.get_item(12));
    println!("Item at key 23: {:?}", tree.get_item(23));
    println!("Item at key 5: {:?}", tree.get_item(5));

    println!("\nPrinting the entire tree:");
    tree.print();
}