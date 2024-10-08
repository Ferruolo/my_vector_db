use std::cmp::{max, min};
use std::mem::swap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use pyo3::ffi::newfunc;
use crate::vector_b_tree::TreeNode::{*};

const ELEMENTS_PER_PAGE: usize = 4;
const MAX_LIVE_PAGES: usize = 8;


type DataType = String;
type IndexType = usize;

/*
 * Base Functions
*/
// Invariants:
// * Depth is equal across

enum TreeNode {
    Null,
    InternalNode(InternalItem),
    LeafNode(LeafItem),
    OverflowNode(Arc<Mutex<TreeNode>>,  IndexType, Arc<Mutex<TreeNode>>),
}

struct LeafItem {
    index: Vec<IndexType>,
    data: Vec<DataType>,
    left_pointer: Arc<Mutex<TreeNode>>,
    right_pointer: Arc<Mutex<TreeNode>>,
}


impl LeafItem {
    fn new() -> Self {
        Self {
            index: vec![],
            data: vec![],
            left_pointer: Arc::new(Mutex::new(Null)),
            right_pointer: Arc::new(Mutex::new(Null)),
        }
    }
}

struct InternalItem {
    index: Vec<IndexType>,
    data: Vec<Arc<Mutex<TreeNode>>>,
    left_pointer: Arc<Mutex<TreeNode>>,
    right_pointer: Arc<Mutex<TreeNode>>,
}

impl InternalItem {
    fn new() -> Self {
        Self {
            index: vec![],
            data: vec![],
            left_pointer: Arc::new(Mutex::new(TreeNode::Null)),
            right_pointer: Arc::new(Mutex::new(TreeNode::Null)),
        }
    }
}



// Comparator returns true if l < r
fn binary_search(data: &Vec<IndexType>, index: &IndexType, comparator: impl Fn(&IndexType, &IndexType) -> bool) -> usize {
    let mut low: usize = 0;
    let mut high: usize = data.len();
    while low < high {
        let mid: usize = (low + high) / 2;
        if comparator(&data[mid], index) {
            high = mid;
        } else {
            low = mid;
        }
    }
    return low;
}

fn compare(l : &IndexType, r: &IndexType) -> bool {
    l < r
}






fn insert_into_leaf_node(mut leaf_item: LeafItem, index: IndexType, data: DataType) -> TreeNode {
    let loc = binary_search(&leaf_item.index, &index, compare);
    leaf_item.index.insert(loc, index);
    leaf_item.data.insert(loc, data);

    if (leaf_item.index.len() > ELEMENTS_PER_PAGE) {
        let mut left = LeafItem::new();
        let mut right = LeafItem::new();
        let midpt = ELEMENTS_PER_PAGE.div_ceil(2);
        let mid_idx = leaf_item.index.get(midpt).unwrap().clone();
        while let (Some(idx), Some(datum)) = (leaf_item.index.pop(), leaf_item.data.pop()) {}


        // Fix Pointers
        left.left_pointer = leaf_item.left_pointer.clone();
        right.right_pointer = leaf_item.right_pointer.clone();

        let left_wrapped = Arc::new(Mutex::new(LeafNode(left)));

        right.left_pointer = left_wrapped.clone();
        let right_wrapped = Arc::new(Mutex::new(LeafNode(right)));

        // This is pretty awkward, please fix?
        match left_wrapped.lock().unwrap().deref_mut() {
            LeafNode(x) => {
                x.right_pointer = right_wrapped.clone();
            }
            _ => panic!(""),
        }
        OverflowNode(left_wrapped, index, right_wrapped)
    } else {
        LeafNode(leaf_item)
    }
}


fn insert_item(node: TreeNode, index: IndexType, data: DataType) -> TreeNode {
    match node {
        Null => {
            let mut leaf_node = LeafItem::new();
            leaf_node.index.push(index);
            leaf_node.data.push(data);
            return LeafNode(leaf_node);
        }
        InternalNode(_) => {}
        LeafNode(x) => {
            return insert_into_leaf_node(x, index, data)
        }
        OverflowNode(_, _, _) => {
            panic!("Never should be inserting into an Overflow node")
        }
    }
    Null
}



fn get_item() {
    
}

fn delete_item() {
    
}

/*
 *  Wrapper!
 */

pub struct BTree {
    root: TreeNode,
    num_elements: usize,
    num_live_pages: usize,
    pub(crate) max_depth: usize
}

impl BTree {
    pub fn new() -> BTree {
        Self {
            root: Null,
            num_elements: 0,
            num_live_pages: 0,
            max_depth: 0
        }
    }

    pub(crate) fn set_item(&mut self, index: IndexType, data: DataType) {
        
    }

    pub fn remove(&mut self, index: IndexType) {
    }

    pub fn get_item(&mut self, index: IndexType) -> Option<&DataType> {
        None
    }

    pub fn print(&self) {

    }
    pub fn get_num_elements(&self) -> usize {
        self.num_elements
    }

    pub fn get_depth(&self) -> usize {
        self.max_depth
    }
}

