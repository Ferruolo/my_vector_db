use std::mem::swap;
use crate::vector_b_tree::BranchChildType::Leaf;
use crate::vector_b_tree::TreeNode::{BranchNode, LeafNode, Null, OverflowNode};

const ELEMENTS_PER_PAGE: usize = 4;

type DataType = String;
type IndexType = usize;
// Beautiful, functional code. Amazing, except it isn't totally functional (yet)

// Can we combine these two similar items?
#[derive(Debug)]
struct BranchItem {
    indexes: Vec<IndexType>,
    data: Vec<TreeNode>,
    branch_type: BranchChildType
}

#[derive(Debug)]
enum BranchChildType { // Assert that all types are equal throughout the branch
    Null,
    Leaf,
    Branch
}

fn binary_search<F>(vec: &[IndexType], comparator: F, index: &IndexType) -> usize
where
    F: Fn(&IndexType, &IndexType) -> bool,
{
    let mut low: usize = 0;
    let mut high: usize = vec.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if comparator(&vec[mid], index) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

impl BranchItem {
    fn new() -> Self {
        Self {
            indexes: vec![],
            data: vec![],
            branch_type: BranchChildType::Null,
        }
    }
}

#[derive(Debug)]
struct LeafItem {
    indexes: Vec<IndexType>,
    data: Vec<DataType>,
}

impl LeafItem {
    fn new() -> Self {
        Self {
            indexes: vec![],
            data: vec![],
        }
    }
}


#[derive(Debug)]
enum TreeNode {
    LeafNode(LeafItem),
    BranchNode(BranchItem),
    OverflowNode(Box<TreeNode>, IndexType, Box<TreeNode>),
    Null,
}

fn find_midpoint(a: &IndexType, b: &IndexType) -> IndexType {
    (a + b + 1) / 2
}

impl TreeNode {
    fn print(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        match self {
            LeafNode(leaf) => {
                println!("{}LeafNode:", indent);
                for (idx, data) in leaf.indexes.iter().zip(leaf.data.iter()) {
                    println!("{}  {}: {}", indent, idx, data);
                }
            }
            BranchNode(branch) => {
                println!("{}BranchNode:", indent);
                for (idx, child) in branch.indexes.iter().zip(branch.data.iter()) {
                    child.print(depth + 1);
                    println!("{}  {} ->", indent, idx);
                }
                branch.data.last().unwrap().print(depth + 1);
            }
            OverflowNode(left, index, right) => {
                println!("{}OverflowNode ({})", indent, index);
                println!("{}  Left:", indent);
                left.print(depth + 1);
                println!("{}  Right:", indent);
                right.print(depth + 1);
            }
            Null => println!("{}Null", indent),
        }
    }
}

/*
 *  Insert Helper Functions
*/

fn leaf_item_mitosis(mut node: LeafItem, index: IndexType, data: DataType) -> TreeNode {
    node = leaf_array_insertion(node, index, data);
    let midpt = ELEMENTS_PER_PAGE.div_ceil(2) + 1;
    let mut left = LeafItem::new();
    let mut right = LeafItem::new();

    assert_eq!(node.indexes.len(), node.data.len());
    // We need to take ownership here
    let mut cur_idx = node.indexes.len();
    node.indexes.reverse();
    node.data.reverse();
    while let (Some(idx), Some(d)) = (node.indexes.pop(), node.data.pop()) {
        let new_leaf = if cur_idx > midpt { &mut left } else { &mut right };
        new_leaf.data.push(d);
        new_leaf.indexes.push(idx);
        cur_idx -= 1;
    }

    let new_midpoint = find_midpoint(&left.indexes.last().unwrap(), &right.indexes.first().unwrap());

    OverflowNode(Box::new(LeafNode(left)), new_midpoint, Box::new(LeafNode(right)))
}

//I never know if going with the imperative route or the functional route is better for the compiler
fn leaf_array_insertion(mut node: LeafItem, index: IndexType, data: DataType) -> LeafItem {
    let insert_position = node.indexes.binary_search(&index).unwrap_or_else(|pos| pos);
    node.indexes.insert(insert_position, index);
    node.data.insert(insert_position, data);
    node
}

// Make sure we take ownership here, no borrowing.   <- this may not scale
fn insert_into_leaf_node(node: LeafItem, index: IndexType, data: DataType) -> TreeNode {
    if (node.indexes.contains(&index)) {
        LeafNode(node)
    } else if (node.indexes.len() >= ELEMENTS_PER_PAGE) {
        leaf_item_mitosis(
            node,
            index,
            data
        )
    } else {
        LeafNode(leaf_array_insertion(node, index, data))
    }
}

fn compare_index_type(left: &IndexType, right: &IndexType) -> bool {
    left < right
}

fn insert_into_branch_node(mut node: BranchItem, index: IndexType, data: DataType) -> TreeNode{
    let idx = binary_search(&node.indexes, compare_index_type, &index);
    let mut selected = Null;
    swap(&mut selected, &mut node.data[idx]);
    let result = match insert_item(selected, index, data) {
        BranchNode(node) => {node}
        OverflowNode(left, new_index, right) => {
            match (*left, *right) { 
                (LeafNode(l), LeafNode(r)) => {
                    match node.branch_type {
                        Leaf => {()} // Null Op for Enum
                        _ => {panic!("Wrong Type of merge here????!!")}
                    }
                    let left_idx = binary_search(&node.indexes, compare_index_type, &new_index);
                    node.indexes.insert(left_idx, new_index);
                    // Oh how I hate to code imperatively (jk I'm too dumb not to)
                    node.data[left_idx] =  LeafNode(l);
                    node.data.insert(left_idx + 1, LeafNode(r));
                    node
                }
                (BranchNode(l), BranchNode(r)) => {
                    match node.branch_type {
                        BranchChildType::Branch => {()} // Null Op for Enum
                        _ => {panic!("Wrong Type of merge here????!!")}
                    }
                    let left_idx = binary_search(&node.indexes, compare_index_type, &new_index);
                    node.indexes.insert(left_idx, new_index);
                    // Oh how I hate to code imperatively (jk I'm too dumb not to)
                    node.data[left_idx] =  BranchNode(l);
                    node.data.insert(left_idx + 1, BranchNode(r));
                    node
                }
                _ => {panic!("And you may ask yourself 'How did I get here'")}                
            }
        }
        _ => {panic!("And you may ask yourself 'How did I get here'")}
    };
    BranchNode(result)
}


/*
 * Base Functions
*/

fn insert_item(node: TreeNode, index: IndexType, data: DataType) -> TreeNode {
    let result =  match node {
        LeafNode(node) => {
            insert_into_leaf_node(node, index, data)
        }
        BranchNode(node) => {
            insert_into_branch_node(node, index, data)
        }
        Null => {
            let new_item = LeafItem::new();
            insert_into_leaf_node(new_item, index, data)
        }
        OverflowNode(left, new_index, right) => {
            OverflowNode(left, new_index, right)
        }
    };
    // Can we do this more elegantly please??
    match result {
        LeafNode(x) => {LeafNode(x)}
        BranchNode(x) => {BranchNode(x)}
        OverflowNode(l, idx, r) => {
            let mut new_branch = BranchItem::new();
            new_branch.indexes.push(idx);
            let branch_left = *l;
            let branch_right = *r;
            new_branch.data.push(branch_left);
            new_branch.data.push(branch_right);
            BranchNode(new_branch)
        }
        Null => {Null}
    }
}

// fn delete_item(node: TreeNode, index: IndexType) -> TreeNode {
//     match node {
//         LeafNode(_) => { Null }
//         BranchNode(x) => {
//             let arr_idx = binary_search(&x.indexes, compare_index_type, &index);
//             let mut delete_path = Null;
//             swap(&mut delete_path, &mut x.data[arr_idx]); // This is NOT FUNCTIONAL
//             // I don't know why you'd commit to that ^
//             x.data[arr_idx] = delete_item(delete_path, index);
//
//             BranchNode(x)
//         }
//         Null => { Null }
//         _ => panic!("This should not happen")
//     }
// }

fn get_data(tree_node: &TreeNode, index: IndexType) -> Option<DataType> {
    match tree_node {
        LeafNode(x) => {Some(x.data.get(index).unwrap().clone())}
        BranchNode(x) => {
            let arr_idx = binary_search(&x.indexes, compare_index_type, &index);
            get_data(&x.data[arr_idx], index)
        }
        _ => None,
    }
}

fn set_data(tree_node: &mut TreeNode, index: IndexType, data: DataType) -> bool {
    match tree_node {
        LeafNode(x) => {
            let arr_idx = binary_search(&x.indexes, compare_index_type, &index);
            x.data[arr_idx] = data;
            true
        }
        BranchNode(x) => {
            let arr_idx = binary_search(&x.indexes, compare_index_type, &index);
            set_data(&mut x.data[arr_idx], index, data)
        }
        _ => {
            false
        }
    }
}


/*
    Wrapper!
 */

pub struct BTree {
    root: TreeNode,
    num_elements: usize
}

impl BTree {
    pub fn new() -> BTree {
        Self {
            root: Null,
            num_elements: 0,
        }
    }
    pub fn insert(&mut self, index: IndexType, data: DataType) {
        let mut root_item = Null;
        swap(&mut self.root, &mut root_item);
        self.root = insert_item(root_item, index, data);
        self.num_elements +=1 ;
    }

    pub fn remove(&mut self, index: IndexType) {
        todo!("Haven't Implemented Delete Yet")
    }

    pub fn get_item(&mut self, index: IndexType) -> Option<DataType> {
        get_data(&self.root, index)
    }

    pub fn set_item(&mut self, index: IndexType, data: DataType) {
        set_data(&mut self.root, index, data);
    }

    pub fn print(&self) {
        println!("BTree (num_elements: {}):", self.num_elements);
        self.root.print(0);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_btree() {
        let tree = BTree::new();
        assert_eq!(tree.num_elements, 0);
    }

    #[test]
    fn test_insert_and_get_single_item() {
        let mut tree = BTree::new();
        tree.insert(0, "Hello".to_string());
        tree.print();
        assert_eq!(tree.get_item(0), Some("Hello".to_string()));
    }

    // #[test]
    // fn test_insert_and_get_multiple_items() {
    //     let mut tree = BTree::new();
    //     tree.insert(0, "First".to_string());
    //     tree.insert(1, "Second".to_string());
    //     tree.insert(2, "Third".to_string());
    //
    //     assert_eq!(tree.get_item(0), Some("First".to_string()));
    //     assert_eq!(tree.get_item(1), Some("Second".to_string()));
    //     assert_eq!(tree.get_item(2), Some("Third".to_string()));
    // }
    //
    // #[test]
    // fn test_insert_overwrite() {
    //     let mut tree = BTree::new();
    //     tree.insert(0, "Original".to_string());
    //     tree.insert(0, "Overwritten".to_string());
    //
    //     assert_eq!(tree.get_item(0), Some("Overwritten".to_string()));
    // }
    //
    // #[test]
    // fn test_get_nonexistent_item() {
    //     let mut tree = BTree::new();
    //     tree.insert(0, "Exists".to_string());
    //
    //     assert_eq!(tree.get_item(1), None);
    // }
    //
    // #[test]
    // fn test_set_item() {
    //     let mut tree = BTree::new();
    //     tree.insert(0, "Original".to_string());
    //     tree.set_item(0, "Updated".to_string());
    //
    //     assert_eq!(tree.get_item(0), Some("Updated".to_string()));
    // }
    //
    // #[test]
    // fn test_set_nonexistent_item() {
    //     let mut tree = BTree::new();
    //     tree.set_item(0, "New".to_string());
    //
    //     assert_eq!(tree.get_item(0), Some("New".to_string()));
    // }
    //
    // #[test]
    // fn test_insert_large_index() {
    //     let mut tree = BTree::new();
    //     tree.insert(1000000, "Large Index".to_string());
    //
    //     assert_eq!(tree.get_item(1000000), Some("Large Index".to_string()));
    // }
    //
    // #[test]
    // fn test_insert_and_get_empty_string() {
    //     let mut tree = BTree::new();
    //     tree.insert(0, "".to_string());
    //
    //     assert_eq!(tree.get_item(0), Some("".to_string()));
    // }
    //
    // #[test]
    // fn test_multiple_operations() {
    //     let mut tree = BTree::new();
    //     tree.insert(0, "Zero".to_string());
    //     tree.insert(1, "One".to_string());
    //     tree.set_item(0, "Updated Zero".to_string());
    //     tree.insert(2, "Two".to_string());
    //
    //     assert_eq!(tree.get_item(0), Some("Updated Zero".to_string()));
    //     assert_eq!(tree.get_item(1), Some("One".to_string()));
    //     assert_eq!(tree.get_item(2), Some("Two".to_string()));
    // }
    //
    //
    // #[test]
    // fn test_insert_and_get_1000_sequential_items() {
    //     let mut tree = BTree::new();
    //     for i in 0..1000 {
    //         tree.insert(i, i.to_string());
    //     }
    //
    //     for i in 0..1000 {
    //         assert_eq!(tree.get_item(i), Some(i.to_string()));
    //     }
    // }
    //
    // #[test]
    // fn test_insert_and_get_1000_reverse_order_items() {
    //     let mut tree = BTree::new();
    //     for i in (0..1000).rev() {
    //         tree.insert(i, i.to_string());
    //     }
    //
    //     for i in 0..1000 {
    //         assert_eq!(tree.get_item(i), Some(i.to_string()));
    //     }
    // }
    //
    // #[test]
    // fn test_insert_1000_items_and_overwrite() {
    //     let mut tree = BTree::new();
    //     for i in 0..1000 {
    //         tree.insert(i, format!("Original {}", i));
    //     }
    //
    //     for i in 0..1000 {
    //         tree.insert(i, format!("Updated {}", i));
    //     }
    //
    //     for i in 0..1000 {
    //         assert_eq!(tree.get_item(i), Some(format!("Updated {}", i)));
    //     }
    // }
    //
    // #[test]
    // fn test_insert_1000_items_with_gaps() {
    //     let mut tree = BTree::new();
    //     for i in 0..1000 {
    //         tree.insert(i * 2, i.to_string());
    //     }
    //
    //     for i in 0..1000 {
    //         assert_eq!(tree.get_item(i * 2), Some(i.to_string()));
    //         assert_eq!(tree.get_item(i * 2 + 1), None);
    //     }
    // }
    //
    // #[test]
    // fn test_insert_and_update_1000_items() {
    //     let mut tree = BTree::new();
    //     for i in 0..1000 {
    //         tree.insert(i, format!("Original {}", i));
    //     }
    //
    //     for i in 0..1000 {
    //         tree.set_item(i, format!("Updated {}", i));
    //     }
    //
    //     for i in 0..1000 {
    //         assert_eq!(tree.get_item(i), Some(format!("Updated {}", i)));
    //     }
    // }
    //
    // #[test]
    // fn test_insert_1000_items_random_order() {
    //     use rand::seq::SliceRandom;
    //     let mut rng = rand::thread_rng();
    //     let mut indices: Vec<usize> = (0..1000).collect();
    //     indices.shuffle(&mut rng);
    //
    //     let mut tree = BTree::new();
    //     for &i in &indices {
    //         tree.insert(i, i.to_string());
    //     }
    //
    //     for i in 0..1000 {
    //         assert_eq!(tree.get_item(i), Some(i.to_string()));
    //     }
    // }
    //
    // #[test]
    // fn test_insert_and_get_large_indices() {
    //     let mut tree = BTree::new();
    //     let large_indices = [10000, 100000, 1000000, 10000000];
    //
    //     for &index in &large_indices {
    //         tree.insert(index, format!("Large {}", index));
    //     }
    //
    //     for &index in &large_indices {
    //         assert_eq!(tree.get_item(index), Some(format!("Large {}", index)));
    //     }
    // }

}