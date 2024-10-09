use std::fmt;
use crate::vector_b_tree::TreeNode::*;
use std::mem::swap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

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
    low
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
        
        leaf_item.index.reverse();
        leaf_item.data.reverse();
        
         
        while let (Some(idx), Some(datum)) = (leaf_item.index.pop(), leaf_item.data.pop()) {
            let selected = if leaf_item.index.len() <= midpt {
                &mut left
            } else {
                &mut right
            };
            selected.index.push(idx);
            selected.data.push(datum);
            
        }
        
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
        OverflowNode(left_wrapped, mid_idx, right_wrapped)
    } else {
        LeafNode(leaf_item)
    }
}

fn split_internal_item(mut internal_item: InternalItem) -> TreeNode {
    let mut left = InternalItem::new();
    let mut right = InternalItem::new();
    let midpt = ELEMENTS_PER_PAGE.div_ceil(2);
    let mid_idx = internal_item.index.get(midpt).unwrap().clone();

    internal_item.index.reverse();
    internal_item.data.reverse();

    // Duplicated from above, pls fix thanks
    while let (Some(idx), Some(datum)) = (internal_item.index.pop(), internal_item.data.pop()) {
        let selected = if internal_item.index.len() <= midpt {
            &mut left
        } else {
            &mut right
        };
        selected.index.push(idx);
        selected.data.push(datum);
    }
    // Fix Pointers
    left.left_pointer = internal_item.left_pointer.clone();
    right.right_pointer = internal_item.right_pointer.clone();

    let left_wrapped = Arc::new(Mutex::new(InternalNode(left)));

    right.left_pointer = left_wrapped.clone();
    let right_wrapped = Arc::new(Mutex::new(InternalNode(right)));

    // This is pretty awkward, please fix?
    match left_wrapped.lock().unwrap().deref_mut() {
        LeafNode(x) => {
            x.right_pointer = right_wrapped.clone();
        }
        _ => panic!(""),
    }
    OverflowNode(left_wrapped, mid_idx, right_wrapped)
}


fn insert_into_internal_item(mut internal_item: InternalItem, index: IndexType, data: DataType) -> TreeNode {
    let loc = binary_search(&internal_item.index, &index, compare);
    let mut node_ref = Null;
    swap(internal_item.data[loc].lock().unwrap().deref_mut(), &mut node_ref);
    match insert_item(node_ref, index, data) {
        OverflowNode(l, idx, r) => {
            internal_item.data[loc] = l;
            internal_item.data.insert(loc + 1, r);
            internal_item.index.insert(loc + 1, idx);
        }
        mut x => {
            swap(internal_item.data[loc].lock().unwrap().deref_mut(), &mut x);
        }
    }
    if internal_item.data.len() >= ELEMENTS_PER_PAGE {
        split_internal_item(internal_item)
    } else {
        InternalNode(internal_item)
    }
}



fn insert_item(node: TreeNode, index: IndexType, data: DataType) -> TreeNode {
    match node {

        Null => {
            let leaf_node = LeafItem::new();
            insert_item(LeafNode(leaf_node), index, data)
        }
        InternalNode(x) => {
            insert_into_internal_item(x, index, data)
        }
        LeafNode(x) => {
            insert_into_leaf_node(x, index, data)
        }
        OverflowNode(_, _, _) => {
            panic!("Never should be inserting into an Overflow node")
        }
    }
}



impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print_node(f, 0)
    }
}

impl TreeNode {
    pub fn get_item(&self, index: &IndexType) -> Option<DataType> {
        self.get_item_inner(index)
    }

    fn get_item_inner(&self, index: &IndexType) -> Option<DataType> {
        match self {
            InternalNode(internal) => {
                let loc = binary_search(&internal.index, index, compare);
                internal.data.get(loc)
                    .and_then(|node| node.lock().ok())
                    .and_then(|node| node.get_item_inner(index))
            },
            LeafNode(leaf) => {
                let loc = binary_search(&leaf.index, index, compare);
                leaf.index.get(loc)
                    .and_then(|idx| if idx == index { leaf.data.get(loc).cloned() } else { None })
            },
            OverflowNode(left, pivot, right) => {
                if index < pivot {
                    left.lock().ok().and_then(|node| node.get_item_inner(index))
                } else {
                    right.lock().ok().and_then(|node| node.get_item_inner(index))
                }
            },
            Null => None,
        }
    }
    fn print_node(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        let indent = "  ".repeat(depth);
        match self {
            TreeNode::Null => writeln!(f, "{}Null", indent),
            TreeNode::InternalNode(internal) => {
                writeln!(f, "{}InternalNode:", indent)?;
                for (i, (idx, child)) in internal.index.iter().zip(internal.data.iter()).enumerate() {
                    writeln!(f, "{}  [{}] Key: {}", indent, i, idx)?;
                    if let Ok(child_node) = child.lock() {
                        child_node.print_node(f, depth + 2)?;
                    }
                }
                Ok(())
            },
            TreeNode::LeafNode(leaf) => {
                writeln!(f, "{}LeafNode:", indent)?;
                for (idx, data) in leaf.index.iter().zip(leaf.data.iter()) {
                    writeln!(f, "{}  Key: {}, Value: {}", indent, idx, data)?;
                }
                Ok(())
            },
            TreeNode::OverflowNode(left, pivot, right) => {
                writeln!(f, "{}OverflowNode (Pivot: {}):", indent, pivot)?;
                writeln!(f, "{}  Left:", indent)?;
                if let Ok(left_node) = left.lock() {
                    left_node.print_node(f, depth + 2)?;
                }
                writeln!(f, "{}  Right:", indent)?;
                if let Ok(right_node) = right.lock() {
                    right_node.print_node(f, depth + 2)?;
                }
                Ok(())
            },
        }
    }
}


fn delete_item() {
    panic!("TODO!")
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
        let mut node = Null;
        swap(&mut self.root, &mut node);
        self.root = insert_item(node, index, data);
    }

    pub fn remove(&mut self, index: IndexType) {
    }

    pub fn get_item(&mut self, index: IndexType) -> Option<DataType> {
        return self.root.get_item(&index)
    }

    pub fn print(&self) {
        println!("{}", self);
    }
    pub fn get_num_elements(&self) -> usize {
        self.num_elements
    }

    pub fn get_depth(&self) -> usize {
        self.max_depth
    }
}

impl fmt::Display for BTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BTree:")?;
        writeln!(f, "  Number of elements: {}", self.num_elements)?;
        writeln!(f, "  Number of live pages: {}", self.num_live_pages)?;
        writeln!(f, "  Max depth: {}", self.max_depth)?;
        writeln!(f, "  Tree structure:")?;
        self.root.print_node(f, 1)
    }
}
