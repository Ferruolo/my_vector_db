# Blog post

Today I am implementing a BTree, several times over the course of the day

## The Plan:
I Implement the B+ tree over the course of a 2 hour sprint. 
I then take a 30 minute break, re-evaluate my code, and then
delete everything and start over. 
I am hoping that this will help me to improve my endurance,
think better about how to write beautiful and performant code,
and overall make me a better software engineer. It is basically
the code equivalent of a track workout. Eventually we want to
get to a similar implementation as found here: 
https://github.com/postgres/postgres/tree/master/src/backend/access/nbtree

Here we go!

## Iteration 1:
Started off with defining the structures:
```rust
// Invariants:
// * Depth is equal across

enum TreeNode {
    Null,
    InternalNode(Arc<Mutex<InternalItem>>),
    LeafNode(Arc<Mutex<LeafItem>>),
}

struct LeafItem {
    index: Vec<IndexType>,
    data: Vec<DataType>,
    left_pointer: Arc<Mutex<TreeNode>>,
    right_pointer: Arc<Mutex<TreeNode>>,
}

struct InternalItem {
    index: Vec<IndexType>,
    data: Vec<Arc<Mutex<TreeNode>>>,
    left_pointer: Arc<Mutex<TreeNode>>,
    right_pointer: Arc<Mutex<TreeNode>>,
}

```
I am putting the mutex in there because we should have multiple links to the same object. 
We need a Reference Counter pointer in rust to be able to do this. We also need
to 

```rust
fn binary_search(data: Vec<IndexType>, index: &IndexType, comparator: impl Fn(&IndexType, &IndexType) -> bool) -> usize {
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

```
As I crossed the 1 hour mark, I found myself with the following code:
```rust
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

```
I know this isn't necessarily correct. More importantly, it is kinda atrocious. 
Going to take a lunch break and then start a new iteration where I fix this.

## We're back, timer's going back on
This time, I'm going to use a couple more functions to make things cleaner, and account
for different structures, and make sure we're watching the invariants.


