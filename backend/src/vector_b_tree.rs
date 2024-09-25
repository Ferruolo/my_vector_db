use std::mem::swap;
use axum::routing::get;
use crate::remove_item;
use crate::vector_b_tree::BranchChildType::Leaf;
use crate::vector_b_tree::TreeNode::{BranchNode, LeafNode, Null, OverflowNode};

const ELEMENTS_PER_PAGE: usize = 4;

type DataType = String;
type IndexType = usize;
// Beautiful, functional code. Amazing, except it isn't totally functional (yet)

// Can we combine these two similar items?
struct BranchItem {
    indexes: Vec<IndexType>,
    data: Vec<TreeNode>,
    branch_type: BranchChildType
}

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

enum TreeNode {
    LeafNode(LeafItem),
    BranchNode(BranchItem),
    OverflowNode(Box<TreeNode>, IndexType, Box<TreeNode>),
    Null,
}


/*
 *  Insert Helper Functions
*/

fn leaf_item_mitosis(mut node: LeafItem, index: IndexType, data: DataType) -> TreeNode {
    node = leaf_array_insertion(node, index, data);
    let midpt = ELEMENTS_PER_PAGE.div_ceil(2);
    let mut left = LeafItem::new();
    let mut right = LeafItem::new();

    assert_eq!(node.indexes.len(), node.data.len());
    // We need to take ownership here
    let cur_idx = node.indexes.len() - 1;
    while let (Some(idx), Some(d)) = (node.indexes.pop(), node.data.pop()) {
        let new_leaf = if cur_idx < midpt { &mut left } else { &mut right };
        new_leaf.data.push(d);
        new_leaf.indexes.push(idx);
    }

    OverflowNode(Box::new(LeafNode(left)), 0 /* TODO FIX THIS*/, Box::new(LeafNode(right)))
}

//I never know if going with the imperative route or the functional route is better for the compiler
fn leaf_array_insertion(mut node: LeafItem, index: IndexType, data: DataType) -> LeafItem {
    let insert_position = node.indexes.binary_search(&index).unwrap_or_else(|pos| pos);
    node.indexes.insert(insert_position, index);
    node.data.insert(insert_position, data);
    node
}
//Make sure we take ownership here, no borrowing.   <- this may not scale
fn insert_into_leaf_node(node: LeafItem, index: IndexType, data: DataType) -> TreeNode {
    if (node.indexes.contains(&index)) {
        LeafNode(node)
    } else if (node.indexes.len() >= ELEMENTS_PER_PAGE) {
        leaf_item_mitosis(node, index, data)
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
    
    Null
}


/*
 * Base Functions
*/

fn insert_item(node: TreeNode, index: IndexType, data: DataType) -> TreeNode {
    match node {
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


