# Blog post

Today I am implementing a BTree, several times over the course of the day

## The Plan:
I Implement the B+ tree over the course of a 2 hour sprint. 
I then take a 30 minute break, re-evaluate my code, and then
delete everything and start over. 
I am hoping that this will help me to improve my endurance,
think better about how to write beautiful and performant code,
and overall make me a better software engineer. It is basically
the code equivalent of a track workout

here we go!

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
I am putting the muitex

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

I want to make this functional, probably will compiler optimize everything!