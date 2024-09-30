use crate::vector_b_tree::BranchChildType::{Branch, Leaf};
use crate::vector_b_tree::TreeNode::{BranchNode, LeafNode, Null, OverflowNode};
use std::cmp::{max, min};
use std::mem::swap;

const ELEMENTS_PER_PAGE: usize = 4;
const MAX_LIVE_PAGES: usize = 8;


type DataType = String;
type IndexType = usize;
// Beautiful, functional code. Amazing, except it isn't totally functional (yet)

// Can we combine these two similar items?
#[derive(Debug)]
struct BranchItem {
    indexes: Vec<IndexType>,
    data: Vec<TreeNode>,
    branch_type: BranchChildType,
    num_leafs: usize,
    max_depth: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BranchChildType {
    // Assert that all types are equal throughout the branch
    Null,
    Leaf,
    Branch
}


fn print_branch_type(x: &BranchChildType) -> String {
    match x {
        BranchChildType::Null => { "NULL".parse().unwrap() }
        Leaf => { "LEAF".parse().unwrap() }
        Branch => { "BRANCH".parse().unwrap() }
    }
}


fn binary_search<F>(index_list: &Vec<IndexType>, comparator: F, index: &IndexType) -> IndexType
where
    F: Fn(&IndexType, &IndexType) -> bool,
{
    let mut low: usize = 0;
    let mut high: usize = index_list.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if comparator(&index_list[mid], index) {
            low = mid + 1;
        } else {
            high = mid;
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
            num_leafs: 0,
            max_depth: 0,
        }
    }
}

#[derive(Debug)]
struct LeafItem {
    indexes: Vec<IndexType>,
    data: Vec<DataType>,
    max_depth: usize,
}

impl LeafItem {
    fn new() -> Self {
        Self {
            indexes: vec![],
            data: vec![],
            max_depth: 0
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
    (a + b + 1).div_ceil(2)
}

fn get_max_depth(tree_node: &TreeNode) -> usize {
    match tree_node {
        LeafNode(x) => {
            1
        }
        BranchNode(x) => {
            x.max_depth
        }
        _ => 0
    }
}

impl TreeNode {
    fn print(&self, depth: usize) {
        let indent = "-".repeat(depth);
        match self {
            LeafNode(leaf) => {
                println!("{}LeafNode:", indent);
                for (idx, data) in leaf.indexes.iter().zip(leaf.data.iter()) {
                    println!("{}  {}: {}", indent, idx, data);
                }
            }
            BranchNode(branch) => {
                println!("{}BranchNode({} Type, {} Index Len, {} Data Len, {} Num Leaves):",
                         indent,
                         print_branch_type(&branch.branch_type),
                         branch.indexes.len(),
                         branch.data.len(),
                         branch.num_leafs
                );

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
    let midpt = ELEMENTS_PER_PAGE.div_ceil(2);
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

    let new_midpoint = find_midpoint(&left.indexes.last().unwrap(), &right.indexes.first().unwrap()) - 1;
    OverflowNode(Box::new(LeafNode(left)), new_midpoint, Box::new(LeafNode(right)))
}

fn branch_item_mitosis(mut node: BranchItem) -> TreeNode {
    let midpt = ELEMENTS_PER_PAGE.div_ceil(2);
    let mut left = BranchItem::new();
    let mut right = BranchItem::new();
    node.indexes.reverse();
    node.data.reverse();
    left.branch_type = node.branch_type.clone();
    right.branch_type = node.branch_type;

    let mut cur_idx = node.indexes.len();
    while let Some(idx) = node.indexes.pop() {
        let new_leaf = if cur_idx > midpt { &mut left } else { &mut right };
        new_leaf.indexes.push(idx);
        cur_idx -= 1;
    }


    cur_idx = node.data.len();
    while let Some(datum) = node.data.pop() {
        let new_leaf = if cur_idx > (midpt + 1) { &mut left } else { &mut right };
        new_leaf.num_leafs += get_num_leafs(&datum);
        new_leaf.data.push(datum);

        cur_idx -= 1;
    }

    // This might be a bad assumption, tbd
    let new_midpt = left.indexes.pop().unwrap();

    OverflowNode(Box::new(BranchNode(left)), new_midpt, Box::new(BranchNode(right)))
}

// I never know if going with the imperative route or the functional route is better for the compiler
fn leaf_array_insertion(mut node: LeafItem, index: IndexType, data: DataType) -> LeafItem {
    let insert_position = node.indexes.binary_search(&index).unwrap_or_else(|pos| pos);
    node.indexes.insert(insert_position, index);
    node.data.insert(insert_position, data);
    node
}

// Make sure we take ownership here, no borrowing. <- this may not scale
fn insert_into_leaf_node(mut node: LeafItem, index: IndexType, data: DataType) -> TreeNode {
    let idx = binary_search(&node.indexes, compare_index_type, &index);
    if idx < node.indexes.len() && node.indexes[idx] == index {
        node.data[idx] = data;
        LeafNode(node)
    } else if node.indexes.len() >= ELEMENTS_PER_PAGE {
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

fn get_num_leafs(x: &TreeNode) -> usize {
    match x {
        LeafNode(x) => {x.data.len()}
        BranchNode(x) => {x.num_leafs}
        _ => 0
    }
}


fn insert_into_branch_node(mut node: BranchItem, index: IndexType, data: DataType) -> TreeNode {
    let idx = binary_search(&node.indexes, compare_index_type, &index);

    let mut selected = Null;
    swap(&mut selected, &mut node.data[idx]);
    // println!("Num Leaves Node: {}", node.num_leafs);
    // println!("Num Leaves Edited: {}", get_num_leafs(&selected));
    // println!("{:?}", node);
    node.num_leafs -= get_num_leafs(&selected);
    let result = match insert_item(selected, index, data) {
        BranchNode(x) => {
            node.data[idx] = BranchNode(x);
            node.num_leafs += get_num_leafs(&node.data[idx]);
            node.max_depth = node.data.iter().map(|x| {get_max_depth(x)}).max().unwrap() + 1;
            node
        }
        OverflowNode(left, new_index, right) => {
            node.num_leafs += get_num_leafs(&left);
            node.num_leafs += get_num_leafs(&right);
            match (*left, *right) { 
                (LeafNode(l), LeafNode(r)) => {
                    match node.branch_type {
                        Leaf => {()} // Null Op for Enum
                        _ => {panic!("Wrong Type of merge here????!!")}
                    }
                    let left_idx = binary_search(&node.indexes, compare_index_type, &new_index);
                    node.indexes.insert(left_idx, new_index);
                    // Oh how I hate to code imperatively (jk I'm too dumb not to)
                    node.max_depth =  node.data.iter().map(|x| {get_max_depth(x)}).max().unwrap() + 1;
                    node.data[left_idx] =  LeafNode(l);
                    node.data.insert(left_idx + 1, LeafNode(r));

                    node
                }
                (BranchNode(l), BranchNode(r)) => {
                    match node.branch_type {
                        Branch => {()} // Null Op for Enum
                        _ => {panic!("Wrong Type of merge here????!!")}
                    }

                    let left_idx = binary_search(&node.indexes, compare_index_type, &new_index);
                    node.indexes.insert(left_idx, new_index);
                    // Oh how I hate to code imperatively (jk I'm too dumb not to)
                    let wrapped_l = BranchNode(l);
                    let wrapped_r = BranchNode(r);
                    
                    node.max_depth = node.data.iter().map(|x| {get_max_depth(x)}).max().unwrap() + 1;
                    node.data[left_idx] =  wrapped_l;
                    node.data.insert(left_idx + 1, wrapped_r);
                    
                    node
                }
                _ => {panic!("And you may ask yourself 'How did I get here'")}                
            }
        }
        LeafNode(x) => {
            node.data[idx] = LeafNode(x);
            node.num_leafs += get_num_leafs(&node.data[idx]);
            node
        }
        _ => {panic!("And you may ask yourself 'How did I get here'")}
    };
    if result.data.len() > ELEMENTS_PER_PAGE {
        branch_item_mitosis(result)
    } else {
        BranchNode(result)
    }
}


/*
 * Delete Helper Functions
*/
fn delete_from_leaf_node(mut node: LeafItem, index: IndexType) -> TreeNode {
    let idx = binary_search(&node.indexes, compare_index_type, &index);
    if idx < node.indexes.len() && node.indexes[idx] == index {
        node.indexes.remove(idx);
        node.data.remove(idx);
    }

    if node.indexes.is_empty() {
        Null
    } else {
        LeafNode(node)
    }
}

fn delete_from_branch_node(mut node: BranchItem, index: IndexType) -> TreeNode {
    let idx = binary_search(&node.indexes, compare_index_type, &index);
    let mut selected = Null;
    swap(&mut selected, &mut node.data[idx]);
    node.num_leafs -= get_num_leafs(&selected);
    match delete_item(selected, index) {
        LeafNode(x) => {
            node.data[idx] = LeafNode(x);
            node.num_leafs += get_num_leafs(&node.data[idx]);
            node.max_depth = node.data.iter().map(|x| {get_max_depth(x) }).max().unwrap();
            if node.num_leafs < ELEMENTS_PER_PAGE { // Merge Leafs
                merge_leafs(node)
            } else {
                BranchNode(node)
            }
        }
        BranchNode(x) => {
            node.data[idx] = BranchNode(x);
            node.num_leafs += get_num_leafs(&node.data[idx]);
            node.max_depth = node.data.iter().map(|x| {get_max_depth(x) }).max().unwrap();
            BranchNode(node)
        }
        Null => {
            if node.num_leafs == 0 || node.indexes.len() == 0 || node.data.len() ==0 {
                return Null;
            }
            
            node.data.remove(idx);
            node.indexes.remove(
                min(idx,
                    if !node.indexes.is_empty() { 
                        node.indexes.len() - 1 
                    } else {
                        0 
                    }
                )
            );
            if node.num_leafs > 0 {
                BranchNode(node)
            } else {
                Null
            }
        }
        _ => {
            Null
        }
    }
}

fn copy_leaf_node_data_over(source: LeafItem, target: &mut LeafItem) {
    //  Note that we take ownership to ensure that source is dropped after
    let mut data = source.data;
    let mut indexes = source.indexes;
    indexes.reverse();
    data.reverse();
    while let Some(d) = data.pop() {
        target.data.push(d);
    }
    while let Some(i) = indexes.pop() {
        target.indexes.push(i);
    }
}



fn merge_leafs(mut node: BranchItem) -> TreeNode {
    if node.num_leafs == 0 {
        return Null
    }
    
    let mut leaf = LeafItem::new();
    
    node.data.reverse();
    
    while let Some(item) = node.data.pop() {
        match item {
            LeafNode(x) => {
                copy_leaf_node_data_over(x, &mut leaf);
            }
            BranchNode(x) => {
                match merge_leafs(x) {
                    LeafNode(x) => {
                        copy_leaf_node_data_over(x, &mut leaf);
                    }
                    _ => {panic!("Merge Leafs returned wrong type (idk how)")}
                }
            }
            _ => {
                
                panic!("Non leaf Node in weird place");
            }
        }
    }
    //Edge Case: Maybe this passes over the limit?
    LeafNode(leaf)
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


fn get_data(tree_node: &TreeNode, index: IndexType) -> Option<&DataType> {
    // Should this return reference or copy? TBD
    match tree_node {
        LeafNode(x) => { 
            let idx = binary_search(&x.indexes, compare_index_type, &index);
            if idx < x.indexes.len() && x.indexes[idx] == index {
                x.data.get(idx)
            } else {
                None
            }
        }
        BranchNode(x) => {
            let arr_idx = binary_search(&x.indexes, compare_index_type, &index);
            get_data(&x.data[arr_idx], index)
        }
        _ => None,
    }
}


fn delete_item(tree_node: TreeNode, index: IndexType) -> TreeNode {
    match tree_node {
        LeafNode(node) => {
            delete_from_leaf_node(node, index)
        }
        BranchNode(
            node
        ) => {
            delete_from_branch_node(node, index)
        }
        _ => {Null}
    }
    
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
                    OverflowNode(_, _, _) => {BranchChildType::Null}
                    Null => {
                        BranchChildType::Null
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
                assert_eq!(branch.branch_type, BranchChildType::Branch);
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
