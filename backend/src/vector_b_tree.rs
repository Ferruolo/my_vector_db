use crate::vector_b_tree::TreeNode::{BranchNode, LeafNode, Null};

const ELEMENTS_PER_PAGE: usize = 4;

type DataType = String;
type IndexType = usize;

// Beautiful, functional code. Amazing, except it isn't totally functional (yet)


// Can we combine these two similar items?
struct BranchItem {
    indexes: Vec<IndexType>,
    data: Vec<TreeNode>,
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
    Null,
}

fn leaf_item_mitosis(mut node: LeafItem, index: IndexType, data: DataType) -> TreeNode {
    node = leaf_array_insertion(node, index, data);
    let midpt = node.indexes[ELEMENTS_PER_PAGE.div_ceil(2)];
    let mut left = LeafItem::new();
    let mut right = LeafItem::new();
    // might not have done this right, help!!!
    for (arr_index, arr_data) in node.indexes.iter().zip(node.data) {
        if arr_index < &midpt {
            left.indexes.push(arr_index.clone());
            left.data.push(arr_data);
        } else {
            right.indexes.push(arr_index.clone());
            right.data.push(arr_data);
        }
    }
    let mut new_branch = BranchItem::new();
    new_branch.indexes.push(midpt);
    new_branch.data.push(LeafNode(left));
    new_branch.data.push(LeafNode(right));
    BranchNode(new_branch)
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
        return LeafNode(node)
    } else if (node.indexes.len() >= ELEMENTS_PER_PAGE) {
        return leaf_item_mitosis(node, index, data)
    } else {
        return LeafNode(leaf_array_insertion(node, index, data));
    }
}

fn insert_into_branch_node(node: BranchItem, index: IndexType, data: DataType) -> TreeNode {
    return Null 
}


fn insert_item(node: TreeNode, index: IndexType, data: DataType) -> TreeNode {
    match node {
        LeafNode(node) => {
            insert_into_leaf_node(node, index, data)
        }
        TreeNode::BranchNode(node) => {
            
        }
        Null => {
            let new_item = LeafItem::new();
            insert_into_leaf_node(new_item, index, data)
        }
    }
}
