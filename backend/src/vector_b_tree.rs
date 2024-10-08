use std::cmp::{max, min};
use std::mem::swap;
use std::sync::{Arc, Mutex};
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


fn insert_item(node: TreeNode, index: IndexType, data: DataType) -> TreeNode {
    match node {
        Null => {
            // Tree is empty, simply return a new Leaf Item
            // Still not clean
            let mut new_leaf = LeafItem::new();
            new_leaf.index.push(index);
            new_leaf.data.push(data);
            LeafNode(new_leaf)
        }
        InternalNode(_) => {
            Null
        }
        LeafNode(mut x) => {
            if (x.index.len() < ELEMENTS_PER_PAGE) {
                let loc = binary_search(&x.index, &index, compare);
                if (x.index[loc] == index) {
                    x.data[loc] = data;
                } else {
                    x.index.insert(loc, index);
                    x.data.insert(loc, data);
                }
                LeafNode(x)
            } else {
                let loc = binary_search(&x.index, &index, compare);
                // We need to split
                x.index.insert(loc, index);
                x.data.insert(loc, data);
                let mut left = LeafItem::new();
                let mut right = LeafItem::new();
                let midpt = x.index.len().div_ceil(2); // Should equal 3
                let mid_idx = x.index[midpt];
                let mut leaf_ref = &mut left;
                while let (Some(idx), Some(datum)) = (x.index.pop(), x.data.pop()) {
                    if (x.index.len() == midpt) {
                        leaf_ref = &mut right;
                    }
                    leaf_ref.index.push(idx);
                    leaf_ref.data.push(datum);
                }
                let mut new_internal_node = InternalItem::new();
                new_internal_node.data.push(Arc::new(Mutex::new(LeafNode(left))));
                new_internal_node.data.push(Arc::new(Mutex::new(LeafNode(right))));
                new_internal_node.index.push(mid_idx);
                InternalNode(new_internal_node)
            }
        }
    }
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
        let mut root_item = Null;
        swap(&mut self.root, &mut root_item);
        self.root = match insert_item(root_item, index, data) {
            OverflowNode(l, idx, r) => {
                let mut new_branch = BranchItem::new();
                new_branch.branch_type = match &*l {
                    LeafNode(_) => {Leaf}
                    BranchNode(_) => {Branch}
                    OverflowNode(_, _, _) => { dtype::Null}
                    Null => {
                        dtype::Null
                    }
                };
                new_branch.num_leafs += get_num_leafs(&l);
                new_branch.num_leafs += get_num_leafs(&r);
                new_branch.indexes.push(idx);
                new_branch.max_depth = max(get_max_depth(&l), get_max_depth(&r)) + 1;
                new_branch.data.push(*l);
                new_branch.data.push(*r);
                BranchNode(new_branch)
            }
            x => {
                x
            }
        };
        // Pre-Emptive Set is more optimal
        self.num_elements = get_num_leafs(&self.root);
        self.max_depth = get_max_depth(&self.root);
    }

    pub fn remove(&mut self, index: IndexType) {
        let mut root = Null;
        swap(&mut self.root, &mut root);
        self.root = delete_item(root, index);
        self.num_elements = get_num_leafs(&self.root)
    }

    pub fn get_item(&mut self, index: IndexType) -> Option<&DataType> {
        get_data(&self.root, index)
    }

    pub fn print(&self) {
        println!("{}", "=".repeat(10));
        println!("BTree (num_elements: {}):", self.num_elements);
        self.root.print(0);
        println!("{}", "=".repeat(10));
    }
    pub fn get_num_elements(&self) -> usize {
        self.num_elements
    }

    pub fn get_depth(&self) -> usize {
        self.max_depth
    }
}


#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;
    use rand::seq::SliceRandom;

    #[test]
    fn test_new_btree() {
        let tree = BTree::new();
        assert_eq!(tree.get_num_elements(), 0);
        match tree.root {
            Null => {},
            _ => panic!("New BTree root should be Null"),
        }
    }

    #[test]
    fn test_insert_and_get_single_item() {
        let mut tree = BTree::new();
        tree.set_item(0, "Hello".to_string());
        assert_eq!(tree.get_num_elements(), 1);
        assert_eq!(tree.get_item(0), Some(&"Hello".to_string()));
    }

    #[test]
    fn test_insert_and_get_multiple_items() {
        let mut tree = BTree::new();
        tree.set_item(0, "First".to_string());
        tree.set_item(1, "Second".to_string());
        tree.set_item(2, "Third".to_string());

        assert_eq!(tree.get_num_elements(), 3);
        assert_eq!(tree.get_item(0), Some(&"First".to_string()));
        assert_eq!(tree.get_item(1), Some(&"Second".to_string()));
        assert_eq!(tree.get_item(2), Some(&"Third".to_string()));
    }

    #[test]
    fn test_insert_overwrite() {
        let mut tree = BTree::new();
        tree.set_item(0, "Original".to_string());
        tree.set_item(0, "Overwritten".to_string());

        assert_eq!(tree.get_num_elements(), 1);
        assert_eq!(tree.get_item(0), Some(&"Overwritten".to_string()));
    }

    #[test]
    fn test_get_nonexistent_item() {
        let mut tree = BTree::new();
        tree.set_item(0, "Exists".to_string());

        assert_eq!(tree.get_item(1), None);
    }

    #[test]
    fn test_set_item() {
        let mut tree = BTree::new();
        tree.set_item(0, "Original".to_string());
        tree.set_item(0, "Updated".to_string());

        assert_eq!(tree.get_num_elements(), 1);
        assert_eq!(tree.get_item(0), Some(&"Updated".to_string()));
    }

    #[test]
    fn test_set_nonexistent_item() {
        let mut tree = BTree::new();
        tree.set_item(0, "New".to_string());

        assert_eq!(tree.get_num_elements(), 1);
        assert_eq!(tree.get_item(0), Some(&"New".to_string()));
    }

    #[test]
    fn test_insert_large_index() {
        let mut tree = BTree::new();
        tree.set_item(1000000, "Large Index".to_string());

        assert_eq!(tree.get_num_elements(), 1);
        assert_eq!(tree.get_item(1000000), Some(&"Large Index".to_string()));
    }

    #[test]
    fn test_insert_and_get_empty_string() {
        let mut tree = BTree::new();
        tree.set_item(0, "".to_string());

        assert_eq!(tree.get_num_elements(), 1);
        assert_eq!(tree.get_item(0), Some(&"".to_string()));
    }

    #[test]
    fn test_multiple_operations() {
        let mut tree = BTree::new();
        tree.set_item(0, "Zero".to_string());
        tree.set_item(1, "One".to_string());
        tree.set_item(0, "Updated Zero".to_string());
        tree.set_item(2, "Two".to_string());

        assert_eq!(tree.get_num_elements(), 3);
        assert_eq!(tree.get_item(0), Some(&"Updated Zero".to_string()));
        assert_eq!(tree.get_item(1), Some(&"One".to_string()));
        assert_eq!(tree.get_item(2), Some(&"Two".to_string()));
    }

    #[test]
    fn test_insert_and_get_1000_sequential_items() {
        let mut tree = BTree::new();
        for i in 0..1000 {
            tree.set_item(i, i.to_string());
        }

        assert_eq!(tree.get_num_elements(), 1000);
        for i in 0..1000 {
            assert_eq!(tree.get_item(i), Some(&i.to_string()));
        }
    }

    #[test]
    fn test_insert_and_get_1000_reverse_order_items() {
        let mut tree = BTree::new();
        for i in (0..1000).rev() {
            tree.set_item(i, i.to_string());
        }

        assert_eq!(tree.get_num_elements(), 1000);
        for i in 0..1000 {
            assert_eq!(tree.get_item(i), Some(&i.to_string()));
        }
    }

    #[test]
    fn test_insert_1000_items_and_overwrite() {
        let mut tree = BTree::new();
        for i in 0..1000 {
            tree.set_item(i, format!("Original {}", i));
        }

        for i in 0..1000 {
            tree.set_item(i, format!("Updated {}", i));
        }

        assert_eq!(tree.get_num_elements(), 1000);
        for i in 0..1000 {
            assert_eq!(tree.get_item(i), Some(&format!("Updated {}", i)));
        }
    }

    #[test]
    fn test_insert_1000_items_with_gaps() {
        let mut tree = BTree::new();
        for i in 0..1000 {
            tree.set_item(i * 2, i.to_string());
        }

        assert_eq!(tree.get_num_elements(), 1000);
        for i in 0..1000 {
            assert_eq!(tree.get_item(i * 2), Some(&i.to_string()));
            assert_eq!(tree.get_item(i * 2 + 1), None);
        }
    }

    #[test]
    fn test_insert_and_update_1000_items() {
        let mut tree = BTree::new();
        for i in 0..1000 {
            tree.set_item(i, format!("Original {}", i));
        }

        for i in 0..1000 {
            tree.set_item(i, format!("Updated {}", i));
        }

        assert_eq!(tree.get_num_elements(), 1000);
        for i in 0..1000 {
            assert_eq!(tree.get_item(i), Some(&format!("Updated {}", i)));
        }
    }

    #[test]
    fn test_insert_1000_items_random_order() {
        let mut rng = rand::thread_rng();
        let mut indices: Vec<usize> = (0..1000).collect();
        indices.shuffle(&mut rng);

        let mut tree = BTree::new();
        for &i in &indices {
            tree.set_item(i, i.to_string());
        }

        assert_eq!(tree.get_num_elements(), 1000);
        for i in 0..1000 {
            assert_eq!(tree.get_item(i), Some(&i.to_string()));
        }
    }

    #[test]
    fn test_insert_and_get_large_indices() {
        let mut tree = BTree::new();
        let large_indices = [10000, 100000, 1000000, 10000000];

        for &index in &large_indices {
            tree.set_item(index, format!("Large {}", index));
        }

        assert_eq!(tree.get_num_elements(), large_indices.len());
        for &index in &large_indices {
            assert_eq!(tree.get_item(index), Some(&format!("Large {}", index)));
        }
    }

    #[test]
    fn test_tree_structure() {
        let mut tree = BTree::new();
        for i in 0..20 {
            tree.set_item(i, i.to_string());
        }

        match &tree.root {
            BranchNode(branch) => {
                assert_eq!(branch.branch_type, dtype::Branch);
                assert!(branch.indexes.len() > 1);
                assert_eq!(branch.indexes.len() + 1, branch.data.len());
            },
            _ => panic!("Root should be a BranchNode after inserting 20 items"),
        }
    }

    #[test]
    fn test_leaf_node_capacity() {
        let mut tree = BTree::new();
        for i in 0..ELEMENTS_PER_PAGE {
            tree.set_item(i, i.to_string());
        }

        match &tree.root {
            LeafNode(leaf) => {
                assert_eq!(leaf.indexes.len(), ELEMENTS_PER_PAGE);
                assert_eq!(leaf.data.len(), ELEMENTS_PER_PAGE);
            },
            _ => panic!("Root should be a LeafNode when number of elements <= ELEMENTS_PER_PAGE"),
        }

        // Insert one more element to cause a split
        tree.set_item(ELEMENTS_PER_PAGE, ELEMENTS_PER_PAGE.to_string());

        match &tree.root {
            BranchNode(_) => {},
            _ => panic!("Root should be a BranchNode after splitting"),
        }
    }

    #[test]
    fn test_delete_single_item() {
        let mut tree = BTree::new();
        tree.set_item(0, "Hello".to_string());
        tree.remove(0);
        assert_eq!(tree.get_num_elements(), 0);
        assert_eq!(tree.get_item(0), None);
    }

    #[test]
    fn test_delete_nonexistent_item() {
        let mut tree = BTree::new();
        tree.set_item(0, "Exists".to_string());
        tree.remove(1);
        assert_eq!(tree.get_num_elements(), 1);
        assert_eq!(tree.get_item(0), Some(&"Exists".to_string()));
    }

    #[test]
    fn test_delete_multiple_items() {
        let mut tree = BTree::new();
        tree.set_item(0, "First".to_string());
        tree.set_item(1, "Second".to_string());
        tree.set_item(2, "Third".to_string());

        tree.remove(1);
        assert_eq!(tree.get_num_elements(), 2);
        assert_eq!(tree.get_item(0), Some(&"First".to_string()));
        assert_eq!(tree.get_item(1), None);
        assert_eq!(tree.get_item(2), Some(&"Third".to_string()));

        tree.remove(0);
        assert_eq!(tree.get_num_elements(), 1);
        assert_eq!(tree.get_item(0), None);
        assert_eq!(tree.get_item(2), Some(&"Third".to_string()));
    }

    #[test]
    fn test_delete_and_reinsert() {
        let mut tree = BTree::new();
        tree.set_item(0, "Original".to_string());
        tree.remove(0);
        tree.set_item(0, "Reinserted".to_string());

        assert_eq!(tree.get_num_elements(), 1);
        assert_eq!(tree.get_item(0), Some(&"Reinserted".to_string()));
    }

    #[test]
    fn test_delete_from_leaf_node() {
        let mut tree = BTree::new();
        for i in 0..ELEMENTS_PER_PAGE {
            tree.set_item(i, i.to_string());
        }

        tree.remove(ELEMENTS_PER_PAGE - 1);
        assert_eq!(tree.get_num_elements(), ELEMENTS_PER_PAGE - 1);
        assert_eq!(tree.get_item(ELEMENTS_PER_PAGE - 1), None);
    }

    #[test]
    fn test_delete_causing_merge() {
        let mut tree = BTree::new();
        for i in 0..ELEMENTS_PER_PAGE * 2 {
            tree.set_item(i, i.to_string());
        }

        // Delete items to cause a merge
        for i in 0..ELEMENTS_PER_PAGE {
            tree.remove(i);
        }

        assert_eq!(tree.get_num_elements(), ELEMENTS_PER_PAGE);
        for i in ELEMENTS_PER_PAGE..ELEMENTS_PER_PAGE * 2 {
            assert_eq!(tree.get_item(i), Some(&i.to_string()));
        }
    }

    #[test]
    fn test_delete_1000_items_sequential() {
        let mut tree = BTree::new();
        for i in 0..1000 {
            tree.set_item(i, i.to_string());
        }

        for i in 0..1000 {
            tree.remove(i);
            assert_eq!(tree.get_item(i), None);
        }

        assert_eq!(tree.get_num_elements(), 0);
    }

    #[test]
    fn test_delete_1000_items_reverse_order() {
        let mut tree = BTree::new();
        for i in 0..1000 {
            tree.set_item(i, i.to_string());
        }

        for i in (0..1000).rev() {
            tree.remove(i);
            assert_eq!(tree.get_item(i), None);
        }

        assert_eq!(tree.get_num_elements(), 0);
    }

    #[test]
    fn test_delete_1000_items_random_order() {
        let mut tree = BTree::new();
        let mut rng = rand::thread_rng();
        let mut indices: Vec<usize> = (0..1000).collect();
        indices.shuffle(&mut rng);
        for i in 0..1000 {
            tree.set_item(i, i.to_string());
        }


        for &i in &indices {
            tree.remove(i);
            assert_eq!(tree.get_item(i), None);
        }

        assert_eq!(tree.get_num_elements(), 0);
    }

    #[test]
    fn test_delete_and_reinsert_1000_items() {
        let mut tree = BTree::new();
        for i in 0..1000 {
            tree.set_item(i, format!("Original {}", i));
        }

        for i in 0..1000 {
            tree.remove(i);
            tree.set_item(i, format!("Reinserted {}", i));
        }

        assert_eq!(tree.get_num_elements(), 1000);
        for i in 0..1000 {
            assert_eq!(tree.get_item(i), Some(&format!("Reinserted {}", i)));
        }
    }

    #[test]
    fn test_delete_large_indices() {
        let mut tree = BTree::new();
        let large_indices = [10000, 100000, 1000000, 10000000];

        for &index in &large_indices {
            tree.set_item(index, format!("Large {}", index));
        }

        for &index in &large_indices {
            tree.remove(index);
            assert_eq!(tree.get_item(index), None);
        }

        assert_eq!(tree.get_num_elements(), 0);
    }
    
    #[test]
    fn test_empty_tree_depth() {
        let tree = BTree::new();
        assert_eq!(tree.max_depth, 0);
    }

    #[test]
    fn test_single_item_depth() {
        let mut tree = BTree::new();
        tree.set_item(0, "Hello".to_string());
        assert_eq!(tree.max_depth, 1);
    }

    #[test]
    fn test_multiple_items_same_leaf_depth() {
        let mut tree = BTree::new();
        for i in 0..ELEMENTS_PER_PAGE {
            tree.set_item(i, i.to_string());
        }
        assert_eq!(tree.max_depth, 1);
    }

    #[test]
    fn test_depth_after_split() {
        let mut tree = BTree::new();
        for i in 0..(ELEMENTS_PER_PAGE + 1) {
            tree.set_item(i, i.to_string());
        }
        assert_eq!(tree.max_depth, 2);
    }

    #[test]
    fn test_depth_multiple_splits() {
        let mut tree = BTree::new();
        for i in 0..100 {
            tree.set_item(i, i.to_string());
        }
        assert!(tree.max_depth > 2);
    }

    #[test]
    fn test_depth_after_delete() {
        let mut tree = BTree::new();
        for i in 0..100 {
            tree.set_item(i, i.to_string());
        }
        let depth_before = tree.max_depth;

        for i in 0..50 {
            tree.remove(i);
        }

        assert!(tree.max_depth <= depth_before);
    }

    #[test]
    fn test_depth_large_tree() {
        let mut tree = BTree::new();
        for i in 0..10000 {
            tree.set_item(i, i.to_string());
        }
        assert!(tree.max_depth > 3);
    }

    #[test]
    fn test_depth_random_inserts_and_deletes() {
        let mut tree = BTree::new();
        let mut rng = rand::thread_rng();

        // Insert 1000 random items
        for _ in 0..1000 {
            let key = rng.gen_range(0..10000);
            tree.set_item(key, key.to_string());
        }

        let depth_after_inserts = tree.max_depth;

        // Delete 500 random items
        for _ in 0..500 {
            let key = rng.gen_range(0..10000);
            tree.remove(key);
        }

        assert!(tree.max_depth <= depth_after_inserts);
    }

    #[test]
    fn test_depth_consistency() {
        let mut tree = BTree::new();
        for i in 0..1000 {
            tree.set_item(i, i.to_string());
            assert_eq!(tree.max_depth, get_max_depth(&tree.root));
        }
    }
}
