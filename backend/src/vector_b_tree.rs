use std::ops::Deref;
use std::sync::{Arc, Mutex};
use crate::vector_b_tree::TreeNode::*;

const ITEMS_PER_PAGE: usize = 5;
const MAX_LIVE_ELEMENTS: usize =  20;


enum TreeNode {
    DiskItem(String),
    LeafNode(LeafItem),
    BranchNode(BranchItem),
    Null
}

struct LeafItem {
    indexes: Vec<usize>,
    data: Vec<String>
}

impl LeafItem {
    pub fn new() -> Self {
        Self {
            indexes: vec![],
            data: vec![]
        }
    }
    pub fn add_item(&mut self, index: usize, data: String) {
        self.indexes.push(index);
        self.data.push(data);
    }
}


struct BranchItem {
    indexes: Vec<usize>,
    children: Vec<Arc<Mutex<TreeNode>>>
}

struct VectorBTree {
    root: Arc<Mutex<TreeNode>>,
    num_items: usize,
    items_per_page: usize,
    current_occupancy: usize,
    max_occupancy: usize,
}


fn insert_item(node: Arc<Mutex<TreeNode>>, index: usize, data: String) {
    match &node.lock().unwrap().deref() {
        DiskItem(filepath) => {
            todo!("Add protobufs integration and caching")
        }
        LeafNode(node) => {
            node.add_item(index, data)
        }
        BranchNode(_) => {}
        Null => {
            let mut new_node = LeafItem::new();
            new_node.add_item(index, data);
            new_node;
        }
    }
}


impl VectorBTree {
    pub fn new() -> Self {
        Self {
            root: Arc::new(Mutex::new(Null)),
            num_items: 0,
            items_per_page: ITEMS_PER_PAGE,
            max_occupancy: MAX_LIVE_ELEMENTS,
            current_occupancy: 0,
        }
    }

    pub fn add_item(&mut self, index: usize, text_data: String) {
        self.root = insert_item(self.root)
    }
}


