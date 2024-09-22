use std::mem::swap;
use crate::vector_b_tree::TreeNode::{BranchNode, LeafNode, Null, OverflowNode};

const ELEMENTS_PER_PAGE: usize = 4;

type DataType = String;
type IndexType = usize;
// Beautiful, functional code. Amazing, except it isn't totally functional (yet)

// Can we combine these two similar items?
struct BranchItem {
    indexes: Vec<IndexType>,
    data: Vec<TreeNode>,
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
    // How do we split
    OverflowNode(Box::new(LeafNode(left)), 0 /* TODO FIOX THIS*/, Box::new(LeafNode(right)))
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




fn insert_into_branch_node(mut node: BranchItem, index: IndexType, data: DataType) -> (TreeNode, TreeNode) {
    let idx = binary_search(&node.indexes, |left, right| {left < right}, &index);
    let mut selected = Null;
    swap(&mut selected, &mut node.data[idx]);
    let result = match insert_item(selected, index, data) {
        LeafNode(_) => {}
        BranchNode(_) => {}
        Null => {}
    };
    
    
    
}


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
    }
}
