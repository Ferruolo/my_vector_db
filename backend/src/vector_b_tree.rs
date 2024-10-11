use std::cmp::max;
use crate::vector_b_tree::TreeNode::*;
use std::fmt;
use std::mem::swap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use libc::uid_t;
use crate::vector_b_tree::ChildType::Data;

const ELEMENTS_PER_PAGE: usize = 4;
const MAX_LIVE_PAGES: usize = 8;


type DataType = String;
type IndexType = usize;

/*
 * Base Functions
*/
// Invariants:



enum TreeNode {
    Null,
    InternalNode(InternalItem),
    LeafNode(LeafItem),
    OverflowNode(Arc<Mutex<TreeNode>>, IndexType, Arc<Mutex<TreeNode>>),
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
            left_pointer: Arc::new(Mutex::new(Null)),
            right_pointer: Arc::new(Mutex::new(Null)),
        }
    }
}

enum ChildType {
    Data(DataType),
    Node(Arc<Mutex<TreeNode>>),
}


impl TreeNode {
    pub fn get_item(&self, index: &IndexType) -> Option<DataType> {
        self.get_item_inner(index)
    }

    fn get_item_inner(&self, index: &IndexType) -> Option<DataType> {
        match self {
            InternalNode(internal) => {
                let loc = binary_search_internal_nodes(&internal.index, &index, compare);

                internal.data.get(loc).and_then(|node| node.lock().ok()).and_then(|node| node.get_item_inner(index))
            }
            LeafNode(leaf) => {
                let loc = binary_search_leafs(&leaf.index, index, compare);
                leaf.index.get(loc).and_then(|idx| if idx == index {
                    leaf.data.get(loc).cloned()
                } else {
                    None
                })
            }
            OverflowNode(left, pivot, right) => {
                if index < pivot {
                    left.lock().ok().and_then(|node| node.get_item_inner(index))
                } else {
                    right.lock().ok().and_then(|node| node.get_item_inner(index))
                }
            }
            Null => None,
        }
    }
    fn print_node(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        let indent = "  ".repeat(depth);
        match self {
            Null => writeln!(f, "{}Null", indent),
            InternalNode(internal) => {
                writeln!(f, "{}InternalNode:", indent)?;
                for (i, (idx, child)) in internal.index.iter().zip(internal.data.iter()).enumerate() {
                    writeln!(f, "{}  [{}] Key: {}", indent, i, idx)?;
                    if let Ok(child_node) = child.lock() {
                        child_node.print_node(f, depth + 2)?;
                    }
                }
                // internal.data.last().unwrap().lock().unwrap().print_node(f, depth + 2)?;
                Ok(())
            }
            LeafNode(leaf) => {
                writeln!(f, "{}LeafNode:", indent)?;
                for (idx, data) in leaf.index.iter().zip(leaf.data.iter()) {
                    writeln!(f, "{}  Key: {}, Value: {}", indent, idx, data)?;
                }
                Ok(())
            }
            OverflowNode(left, pivot, right) => {
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
            }
        }
    }

    fn get_max_depth(&self, prev_depth: usize) -> usize {
        match self {
            InternalNode(x) => {
                x.data.iter().map(|n| {
                    n.lock().unwrap().get_max_depth(prev_depth + 1)
                }).max().unwrap()
            }
            LeafNode(_) => { prev_depth + 1 }
            _ => { prev_depth }
        }
    }

    fn iterate_through(&self, mut accum: Vec<DataType>) -> Vec<DataType> {
        match self {
            InternalNode(node) => {
                node.data.first().unwrap().lock().unwrap().iterate_through(accum)
            }
            LeafNode(node) => {
                accum.extend(node.data.clone());
                node.right_pointer.clone().lock().unwrap().iterate_through(accum)
            }
            _ => {
                accum
            }
        }
    }

    fn set_left_pointer(&mut self, new_ptr: Arc<Mutex<TreeNode>>) {
        match self {
            Null => {}
            InternalNode(x) => {
                x.left_pointer = new_ptr.clone();
            }
            LeafNode(x) => {
                x.left_pointer = new_ptr.clone();
            }
            OverflowNode(_, _, _) => {}
        }
    }

    fn set_right_pointer(&mut self, new_ptr: Arc<Mutex<TreeNode>>) {
        match self {
            Null => {}
            InternalNode(x) => {
                x.right_pointer = new_ptr.clone(); // Clone because safer (idk doesn't really make a difference)
            }
            LeafNode(x) => {
                x.right_pointer = new_ptr.clone();
            }
            OverflowNode(_, _, _) => {}
        }
    }

    fn is_null(&self) -> bool {
        match self {
            Null => { true }
            _ => { false }
        }
    }

    fn pass_right(&mut self) -> Option<(IndexType, ChildType)> {
        match self {
            Null => {
                None
            }
            OverflowNode(_, _, _) => {
                panic!("Overflow node should never be linked to")
            }

            LeafNode(x) => {
                if x.index.len() <= ELEMENTS_PER_PAGE / 2 {
                    None
                } else {
                    let (last_idx) = x.index.pop().unwrap();
                    let (last_data) = x.data.pop().unwrap();
                    Some((last_idx, Data(last_data)))
                }
            }
            InternalNode(x) => {
                if x.index.len() <= ELEMENTS_PER_PAGE / 2 {
                    None
                } else {
                    let (last_idx) = x.index.pop().unwrap();
                    let (last_data) = x.data.pop().unwrap();
                    Some((last_idx, ChildType::Node(last_data)))
                }
            }
        }
    }

    fn pass_left(&mut self) -> Option<(IndexType, ChildType)> {
        match self {
            Null => {
                None
            }
            InternalNode(_) => {
                None
            }
            LeafNode(x) => {
                if x.index.len() <= ELEMENTS_PER_PAGE / 2 {
                    None
                } else {
                    let (last_idx) = x.index.remove(0);
                    let (last_data) = x.data.remove(0);
                    Some((last_idx, Data(last_data)))
                }
            }
            OverflowNode(_, _, _) => {
                panic!("Overflow node should never be linked to")
            }
        }
    }

    fn merge_from_right(&mut self, node: TreeNode) {
        println!("Merge from right");
        match node {
            InternalNode(mut x) => {
                match self {
                    InternalNode(current_node) => {
                        while let (Some(idx), Some(datum)) = (x.index.pop(), x.data.pop()) {
                            current_node.index.push(idx);
                            current_node.data.push(datum);

                        }
                        current_node.right_pointer = x.right_pointer.clone();
                        x.right_pointer.lock().unwrap().set_left_pointer(x.left_pointer.clone());

                    }
                    _ => {panic!("Can't merge internal nodes with non leaf internal")}
                }
            }
            LeafNode(mut x) => {
                match self {
                    LeafNode(current_node) => {
                        while let (Some(idx), Some(datum)) = (x.index.pop(), x.data.pop()) {
                            current_node.index.push(idx);
                            current_node.data.push(datum);

                        }
                        current_node.right_pointer = x.right_pointer.clone();
                        x.right_pointer.lock().unwrap().set_left_pointer(x.left_pointer.clone());

                    }
                    _ => {panic!("Can't merge leaf nodes with non leaf nodes")}
                }
            }
            Null => {}
            _ => {panic!("Can't merge an overflow using merge_from_right")}
        }
    }

    fn merge_from_left(&mut self, node: TreeNode) {
        println!("Merge from left");
        match node {
            InternalNode(x) => {
                match self {
                    InternalNode(current_node) => {
                        // note that these take ownership (or should at least)
                        let mut index = x.index;
                        let mut data = x.data;

                        match index.last() {
                            None => {}
                            Some(x) => {
                                if *x == usize::max_value() {
                                    index.pop();
                                }
                            }
                        }

                        current_node.index.reverse();
                        current_node.data.reverse();
                        while let (Some(idx), Some(datum)) = (current_node.index.pop(), current_node.data.pop()) {
                            index.push(idx);
                            data.push(datum);
                        }
                        current_node.index = index;
                        current_node.data = data;
                        current_node.left_pointer = x.left_pointer.clone();
                        x.left_pointer.lock().unwrap().set_right_pointer(x.right_pointer.clone());
                    }
                    _ => {panic!("Can't merge internal nodes with non internal nodes")}
                }
            }
            LeafNode(mut x) => {
                match self {
                    LeafNode(current_node) => {
                        // note that these take ownership (or should at least)
                        let mut index = x.index;
                        let mut data = x.data;
                        current_node.index.reverse();
                        current_node.data.reverse();
                        while let (Some(idx), Some(datum)) = (current_node.index.pop(), current_node.data.pop()) {
                            index.push(idx);
                            data.push(datum);
                        }
                        current_node.index = index;
                        current_node.data = data;
                        current_node.left_pointer = x.left_pointer.clone();
                        x.left_pointer.lock().unwrap().set_right_pointer(x.right_pointer.clone());

                    }
                    _ => {panic!("Can't merge leaf nodes with non leaf nodes")}
                }
            }
            Null => {}
            _ => {panic!("Can't merge an overflow using merge_from_right")}
        }
    }
}

trait NodeInterface {
    fn reverse_data(&mut self);
    fn pop_last_data_and_index(&mut self)-> Option<(IndexType, ChildType)>;

    fn get_left_pointer(&self) -> Arc<Mutex<TreeNode>>;

    fn get_right_pointer(&self) -> Arc<Mutex<TreeNode>>;
    fn insert (&mut self, index: IndexType, datum: Data, loc: usize);
    fn get_loc(&self, index: &IndexType) -> usize;
}


impl NodeInterface for LeafItem{
    fn reverse_data(&mut self) {
        self.index.reverse();
        self.data.reverse();
    }

    fn pop_last_data_and_index(&mut self) -> Option<(IndexType, ChildType)> {
        match (self.index.pop(), self.data.pop()) {
            (Some(idx), Some(datum)) => {
                Some((idx, Data(datum)))
            }
            (_, _) => {None}
        }
    }

    fn get_left_pointer(&self) -> Arc<Mutex<TreeNode>> {
        self.left_pointer.clone()
    }

    fn get_right_pointer(&self) -> Arc<Mutex<TreeNode>> {
        self.right_pointer.clone()
    }

    fn insert (&mut self, index: IndexType, datum: Data, loc: usize) {
        self.index.insert(loc, index);
        self.data.insert(loc, datum);
    }

    fn get_loc(&self, index: &IndexType) -> usize {
        binary_search_leafs(&self.index, &index, compare)
    }

}


// Comparator returns true if l < r
fn binary_search_leafs(data: &Vec<IndexType>, index: &IndexType, comparator: impl Fn(&IndexType, &IndexType) -> bool) -> usize {
    if data.len() == 0 {
        return 0;
    }

    let mut low: usize = 0;
    let mut high: usize = data.len();
    while low < high {
        let mid: usize = low + (high - low) / 2;

        if comparator(index, &data[mid]) {
            high = mid;
        } else if !comparator(&data[mid], index) {
            return mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

fn binary_search_internal_nodes(data: &Vec<IndexType>, index: &IndexType, comparator: impl Fn(&IndexType, &IndexType) -> bool) -> usize {
    let mut low: usize = 0;
    let mut high: usize = data.len();
    while low < high {
        let mid: usize = low + (high - low) / 2;
        if comparator(index, &data[mid]) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    high
}


fn compare(l: &IndexType, r: &IndexType) -> bool {
    l < r
}




fn insert_into_leaf_node<T: NodeInterface>(mut item: T, index: IndexType, data: DataType) -> TreeNode {
    let loc = item.get_loc(&index);
    item.insert(index, data, loc);


    if (item.index.len() > ELEMENTS_PER_PAGE) {
        let mut left = LeafItem::new();
        let mut right = LeafItem::new();
        let midpt = ELEMENTS_PER_PAGE.div_ceil(2);
        let mid_idx = item.index.get(midpt).unwrap().clone();

        item.index.reverse();
        item.data.reverse();

        while let (Some(idx), Some(datum)) = (item.index.pop(), item.data.pop()) {
            let selected = if item.index.len() > midpt {
                &mut left
            } else {
                &mut right
            };
            selected.index.push(idx);
            selected.data.push(datum);
        }

        // Fix Pointers
        left.left_pointer = item.left_pointer.clone();
        right.right_pointer = item.right_pointer.clone();

        let left_wrapped = Arc::new(Mutex::new(LeafNode(left)));
        let right_wrapped = Arc::new(Mutex::new(LeafNode(right)));

        // This is pretty awkward, please fix?
        right_wrapped.lock().unwrap().set_left_pointer(left_wrapped.clone());
        left_wrapped.lock().unwrap().set_right_pointer(right_wrapped.clone());

        item.left_pointer.lock().unwrap().set_right_pointer(left_wrapped.clone());
        item.right_pointer.lock().unwrap().set_left_pointer(right_wrapped.clone());
        OverflowNode(left_wrapped, mid_idx, right_wrapped)
    } else {
        LeafNode(item)
    }
}

fn insert_into_internal_item(mut internal_item: InternalItem, index: IndexType, data: DataType) -> TreeNode {
    let loc = binary_search_leafs(&internal_item.index, &index, compare);
    let mut node_ref = Null;
    swap(internal_item.data[loc].lock().unwrap().deref_mut(), &mut node_ref);
    match insert_item(node_ref, index, data) {
        OverflowNode(l, idx, r) => {
            internal_item.data[loc] = l;
            internal_item.data.insert(loc + 1, r);
            internal_item.index.insert(loc, idx);
        }
        mut x => {
            swap(internal_item.data[loc].lock().unwrap().deref_mut(), &mut x);
        }
    }
    if internal_item.data.len() > ELEMENTS_PER_PAGE {
        split_internal_item(internal_item)
    } else {
        InternalNode(internal_item)
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
        let selected = if internal_item.index.len() < midpt {
            &mut right
        } else {
            &mut left
        };
        selected.index.push(idx);
        selected.data.push(datum);
    }
    left.index.push(usize::max_value());
    // Fix Pointers
    left.left_pointer = internal_item.left_pointer.clone();
    right.right_pointer = internal_item.right_pointer.clone();

    let left_wrapped = Arc::new(Mutex::new(InternalNode(left)));

    right.left_pointer = left_wrapped.clone();
    let right_wrapped = Arc::new(Mutex::new(InternalNode(right)));

    // This is pretty awkward, please fix?
    match left_wrapped.lock().unwrap().deref_mut() {
        InternalNode(x) => {
            x.right_pointer = right_wrapped.clone();
        }
        _ => panic!(""),
    }
    OverflowNode(left_wrapped, mid_idx, right_wrapped)
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

fn delete_from_leaf_item(mut node: LeafItem, index: IndexType) -> TreeNode {
    let loc = binary_search_leafs(&node.index, &index, compare);
    if loc < node.index.len() && node.index[loc] == index {
        node.index.remove(loc);
        node.data.remove(loc);

        if node.index.is_empty() {
            return Null;
        } else if node.index.len() < ELEMENTS_PER_PAGE / 2 {
            // merge if underfull
            if let Some((new_idx, new_datum)) = node.left_pointer.clone().lock().unwrap().pass_right() {
                // Try to take from left
                node.index.insert(0, new_idx);
                node.data.insert(0, match new_datum {
                    Data(x) => {x}
                    ChildType::Node(_) => {
                        panic!("Internal links to a leaf node")
                    }
                });
                return LeafNode(node)
            }

            if let Some((new_idx, new_datum)) = node.right_pointer.clone().lock().unwrap().pass_left() {
                // Try to take from right
                node.index.push(new_idx);
                node.data.push(match new_datum {
                    Data(x) => {x}
                    ChildType::Node(_) => {
                        panic!("Internal links to a leaf node")
                    }
                });
                return LeafNode(node)
            }

            if !node.left_pointer.clone().lock().unwrap().is_null() {
                // Merge to Left
                node.left_pointer.clone().lock().unwrap().merge_from_right(LeafNode(node));
                Null
            } else if !node.right_pointer.lock().unwrap().is_null() {
                node.right_pointer.clone().lock().unwrap().merge_from_left(LeafNode(node));
                Null
            } else {
                LeafNode(node)
            }
        } else {
            LeafNode(node)
        }
    } else {
        // Does not contain record
        LeafNode(node)
    }
}

fn delete_from_internal_node(mut node: InternalItem, index: IndexType) -> TreeNode {
    let loc = binary_search_internal_nodes(&node.index, &index, compare);
    let mut selected_child = Null;

    if loc >= node.index.len() {
        return InternalNode(node);
    }
    swap(&mut selected_child, node.data[loc].lock().unwrap().deref_mut());
    match delete_item(selected_child, index) {
        Null => {
            node.data.remove(loc);

            node.index.remove(max(loc, 1) - 1);
        }
        InternalNode(x) => {
            selected_child = InternalNode(x);
            swap(&mut selected_child, node.data[loc].lock().unwrap().deref_mut());
        }
        LeafNode(x) => {
            if (node.index.len() > 1 && loc > 0 && *x.index.first().unwrap() < node.index[loc - 1]) {
                node.index[loc - 1] = *x.index.first().unwrap();
            }
            selected_child = LeafNode(x);
            swap(&mut selected_child, node.data[loc].lock().unwrap().deref_mut());
        }
        OverflowNode(_, _, _) => {
            panic!("Never should be delete into an Overflow node")
        }
    }

    if node.data.len() < ELEMENTS_PER_PAGE / 2 {
        // merge if underfull
        if let Some((new_idx, new_datum)) = node.left_pointer.clone().lock().unwrap().pass_right() {
            // Try to take from left
            node.index.insert(0, new_idx);
            node.data.insert(0, match new_datum {
                Data(_) => {panic!("Internal links to a leaf node")}
                ChildType::Node(x) => {x}
            });
            return InternalNode(node)
        }

        if let Some((new_idx, new_datum)) = node.right_pointer.clone().lock().unwrap().pass_left() {
            // Try to take from right
            node.index.push(new_idx);
            node.data.push(match new_datum {
                Data(_) => {panic!("Internal links to a leaf node")}
                ChildType::Node(x) => {x}
            });
            return InternalNode(node)
        }

        if !node.left_pointer.clone().lock().unwrap().is_null() {
            // Merge to Left
            node.left_pointer.clone().lock().unwrap().merge_from_right(InternalNode(node));
            Null
        } else if !node.right_pointer.lock().unwrap().is_null() {
            node.right_pointer.clone().lock().unwrap().merge_from_left(InternalNode(node));
            Null
        } else {
            if (node.data.len() == 1) {
                let mut child = Null;
                swap(&mut child, node.data[0].lock().unwrap().deref_mut());
                child
            } else {
                InternalNode(node)
            }
        }
    } else {
        InternalNode(node)
    }
}


fn delete_item(node: TreeNode, index: IndexType) -> TreeNode {
    match node {
        InternalNode(mut x) => {
            delete_from_internal_node(x, index)
        }
        LeafNode(x) => {
            delete_from_leaf_item(x, index)
        }
        _ => Null
    }
}

/*
 *  Wrapper!
 */

pub struct BTree {
    root: TreeNode,
    num_elements: usize,
    num_live_pages: usize,
    pub(crate) max_depth: usize,
}

impl BTree {
    pub fn new() -> BTree {
        Self {
            root: Null,
            num_elements: 0,
            num_live_pages: 0,
            max_depth: 0,
        }
    }

    pub(crate) fn set_item(&mut self, index: IndexType, data: DataType) {
        let mut node = Null;
        swap(&mut self.root, &mut node);
        self.root = match insert_item(node, index, data) {
            OverflowNode(l, idx, r) => {
                let mut new_root = InternalItem::new();
                new_root.index.push(idx);
                new_root.index.push(usize::max_value());
                new_root.data.push(l);
                new_root.data.push(r);
                InternalNode(new_root)
            }
            other => other
        };
    }

    pub fn remove(&mut self, index: IndexType) {
        let mut node = Null;
        swap(&mut self.root, &mut node);
        self.root = delete_item(node, index);
    }

    pub fn get_item(&self, index: IndexType) -> Option<DataType> {
        return self.root.get_item(&index);
    }

    pub fn print(&self) {
        println!("{}", self);
    }
    pub fn get_num_elements(&self) -> usize {
        self.num_elements
    }

    pub fn get_depth(&self) -> usize {
        self.root.get_max_depth(0)
    }

    pub fn iterate_through(&self) -> Vec<DataType> {
        self.root.iterate_through(vec![])
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

// TODO: I need to write better/more extensive tests
#[cfg(test)]
mod tests {
    use crate::vector_b_tree::{BTree, DataType};
    // Each test function is annotated with #[test]
    #[test]
    fn init_test() {
        let tree = BTree::new();
        assert_eq!(tree.get_num_elements(), 0);
        assert_eq!(tree.get_depth(), 0);
        assert_eq!(tree.get_item(0), None);
    }

    #[test]
    fn one_leaf() {
        let mut tree = BTree::new();
        let strings: Vec<String> = vec![
            String::from("E"),
            String::from("F"),
            String::from("T"),
            String::from("Q")
        ];


        tree.set_item(9, strings[0].clone());
        tree.set_item(10, strings[1].clone());
        tree.set_item(12, strings[2].clone());
        tree.set_item(23, strings[3].clone());
        // assert_eq!(tree.get_num_elements(), 4);
        assert_eq!(tree.get_depth(), 1);
        assert_eq!(tree.get_item(9), Some(strings[0].clone()));
        assert_eq!(tree.get_item(10), Some(strings[1].clone()));
        assert_eq!(tree.get_item(11), None);
        assert_eq!(tree.get_item(12), Some(strings[2].clone()));
        assert_eq!(tree.get_item(23), Some(strings[3].clone()));
    }

    #[test]
    fn split_leafs() {
        let mut tree = BTree::new();
        let strings: Vec<String> = vec![
            String::from("E"),
            String::from("G"),
            String::from("T"),
            String::from("Q"),
            String::from("F")
        ];


        tree.set_item(9, strings[0].clone());
        tree.set_item(10, strings[1].clone());
        tree.set_item(12, strings[2].clone());
        tree.set_item(23, strings[3].clone());
        tree.set_item(5, strings[4].clone());
        // assert_eq!(tree.get_num_elements(), 4);
        assert_eq!(tree.get_depth(), 2);
        assert_eq!(tree.get_item(9), Some(strings[0].clone()));
        assert_eq!(tree.get_item(10), Some(strings[1].clone()));
        assert_eq!(tree.get_item(11), None);
        assert_eq!(tree.get_item(12), Some(strings[2].clone()));
        assert_eq!(tree.get_item(23), Some(strings[3].clone()));
        assert_eq!(tree.get_item(5), Some(strings[4].clone()));
    }

    #[test]
    fn split_leafs_with_internal_node() {
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

        assert_eq!(tree.get_depth(), 2);
        assert_eq!(tree.get_item(9), Some(strings[0].clone()));
        assert_eq!(tree.get_item(10), Some(strings[1].clone()));
        assert_eq!(tree.get_item(11), None);
        assert_eq!(tree.get_item(12), Some(strings[2].clone()));
        assert_eq!(tree.get_item(23), Some(strings[3].clone()));
        assert_eq!(tree.get_item(5), Some(strings[4].clone()));
    }

    #[test]
    fn split_internal_node() {
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


        assert_eq!(tree.get_depth(), 3);
        assert_eq!(tree.get_item(9), Some(strings[0].clone()));
        assert_eq!(tree.get_item(10), Some(strings[1].clone()));
        assert_eq!(tree.get_item(11), None);
        assert_eq!(tree.get_item(12), Some(strings[2].clone()));
        assert_eq!(tree.get_item(23), Some(strings[3].clone()));
        assert_eq!(tree.get_item(5), Some(strings[4].clone()));
    }

    #[test]
    fn test_iterate() {
        let mut tree = BTree::new();
        let strings: Vec<DataType> = vec![
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

        tree.set_item(0, strings[11].clone());
        tree.set_item(1, strings[12].clone());
        tree.set_item(50, strings[13].clone());
        tree.set_item(55, strings[14].clone());

        let values: Vec<DataType> = vec![
            "H", "L", "B",
            "F", "A", "E",
            "G", "T",
            "Q", "A",
            "F", "V",
            "V", "Alpha", "Omega"
        ].iter().map(|d| d.to_string()).collect();


        let iterate = tree.iterate_through();
        assert_eq!(iterate.len(), values.len());
        for (left, right) in values.iter().zip(iterate) {
            assert_eq!(*left, right);
        }
    }

    #[test]
    fn delete_one_leaf() {
        let mut tree = BTree::new();
        let strings: Vec<String> = vec![
            String::from("E"),
            String::from("F"),
            String::from("T"),
            String::from("Q")
        ];


        tree.set_item(9, strings[0].clone());
        tree.set_item(10, strings[1].clone());
        tree.set_item(12, strings[2].clone());
        tree.set_item(23, strings[3].clone());
        // assert_eq!(tree.get_num_elements(), 4);
        assert_eq!(tree.get_depth(), 1);
        assert_eq!(tree.get_item(9), Some(strings[0].clone()));
        assert_eq!(tree.get_item(10), Some(strings[1].clone()));
        assert_eq!(tree.get_item(11), None);
        assert_eq!(tree.get_item(12), Some(strings[2].clone()));
        assert_eq!(tree.get_item(23), Some(strings[3].clone()));

        tree.remove(9);
        tree.remove(10);
        tree.remove(12);
        tree.remove(23);

        assert_eq!(tree.get_depth(), 0);
        assert_eq!(tree.get_item(9), None);
        assert_eq!(tree.get_item(10), None);
        assert_eq!(tree.get_item(12), None);
        assert_eq!(tree.get_item(23), None);
    }


    #[test]
    fn delete_no_merge() {
        let mut tree = BTree::new();
        let strings: Vec<String> = vec![
            String::from("E"),
            String::from("G"),
            String::from("T"),
            String::from("Q"),
            String::from("F")
        ];


        tree.set_item(9, strings[0].clone());
        tree.set_item(10, strings[1].clone());
        tree.set_item(12, strings[2].clone());
        tree.set_item(23, strings[3].clone());
        tree.set_item(5, strings[4].clone());
        // assert_eq!(tree.get_num_elements(), 4);
        tree.remove(12);
        assert_eq!(tree.get_depth(), 2);
        assert_eq!(tree.get_item(9), Some(strings[0].clone()));
        assert_eq!(tree.get_item(23), Some(strings[3].clone()));
        assert_eq!(tree.get_item(12), None);
    }

    #[test]
    fn delete_one_from_far_left_no_merge() {
        let mut tree = BTree::new();
        let strings: Vec<String> = vec![
            String::from("E"),
            String::from("G"),
            String::from("T"),
            String::from("Q"),
            String::from("F")
        ];


        tree.set_item(9, strings[0].clone());
        tree.set_item(10, strings[1].clone());
        tree.set_item(12, strings[2].clone());
        tree.set_item(23, strings[3].clone());
        tree.set_item(5, strings[4].clone());
        // assert_eq!(tree.get_num_elements(), 4);
        assert_eq!(tree.get_item(5), Some(strings[4].clone()));
        tree.remove(5);
        assert_eq!(tree.get_depth(), 2);
        assert_eq!(tree.get_item(9), Some(strings[0].clone()));
        assert_eq!(tree.get_item(23), Some(strings[3].clone()));
        assert_eq!(tree.get_item(12), Some(strings[2].clone()));
        assert_eq!(tree.get_item(5), None);
    }

    #[test]
    fn delete_two_from_far_left_with_merge() {
        let mut tree = BTree::new();
        let strings: Vec<String> = vec![
            String::from("E"),
            String::from("G"),
            String::from("T"),
            String::from("Q"),
            String::from("F")
        ];

        tree.set_item(9, strings[0].clone());
        tree.set_item(10, strings[1].clone());
        tree.set_item(12, strings[2].clone());
        tree.set_item(23, strings[3].clone());
        tree.set_item(5, strings[4].clone());
        // assert_eq!(tree.get_num_elements(), 4);
        assert_eq!(tree.get_item(5), Some(strings[4].clone()));
        assert_eq!(tree.get_item(9), Some(strings[0].clone()));
        tree.remove(5);
        tree.remove(9);
        assert_eq!(tree.get_depth(), 1);
        assert_eq!(tree.get_item(9), None);
        assert_eq!(tree.get_item(23), Some(strings[3].clone()));
        assert_eq!(tree.get_item(12), Some(strings[2].clone()));
        assert_eq!(tree.get_item(5), None);
    }

    #[test]
    fn something_i_found() {
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

        tree.set_item(0, strings[11].clone());
        tree.set_item(1, strings[12].clone());
        tree.set_item(50, strings[13].clone());
        tree.set_item(55, strings[14].clone());

        tree.print();
        tree.remove(5);
        tree.remove(9);
        tree.print();
        tree.remove(10);
        tree.remove(7);
        tree.print();
        tree.set_item(22, "A".to_string());
    }
}
