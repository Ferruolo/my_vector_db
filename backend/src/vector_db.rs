use crate::helpers::{binary_search, compare};
use crate::llama_embedding::LlamafileEmbedding;
use crate::vector_db::TreeNode::{LeafNode, Null};
use tch::{Device, Tensor};

const ELEMENTS_PER_PAGE: usize = 10;

struct Node<T> {
    data: Vec<T>,
    indexes: Vec<Tensor>,
}

enum TreeNode<T> {
    LeafNode(Node<T>),
    Null,
    OverflowNode(Box<TreeNode<T>>, Tensor, Box<TreeNode<T>>),
}

struct VectorDB<T> {
    data: Vec<TreeNode<T>>,
    indexes: Vec<Tensor>,
    embedding_item: LlamafileEmbedding,
    zero: Tensor,
}


fn split_node(node: Node<T>) {
    let mid
}


fn insert_into_tree_node<T>(
    node: &mut TreeNode<T>,
    new_data: T,
    query: Tensor,
    compare: impl Fn(&Tensor, &Tensor) -> bool,
) {
    match node {
        LeafNode(node) => {
            let loc = binary_search(&node.indexes, &query, &compare);
            node.data.insert(loc, new_data);
            node.indexes.insert(loc, query);
            split_node(node);
        }
        Null => {
            todo!();
        }
        TreeNode::OverflowNode(_, _, _) => {
            panic!("Should never be inserting into an overflow node")
        }
    }
}

impl<T> VectorDB<T> {
    pub fn new(model_path: &str, dims: usize) -> Self {
        let zero = Tensor::zeros(&[dims as i64], (tch::Kind::Float, Device::Cuda(0)));
        let embedding_model = LlamafileEmbedding::new(model_path);
        Self {
            data: vec![],
            embedding_item: embedding_model,
            zero: zero,
            indexes: vec![], // compare: Box::ne,
        }
    }

    pub fn insert(&mut self, new_data: T, index_string: String) {
        let query = self.embedding_item.get_embedding(&index_string);
        let compare_func = compare(&self.zero);
        if self.data.is_empty() {
            self.data.push(LeafNode(Node {
                data: vec![new_data],
                indexes: vec![query],
            }));
        } else {
            let loc = binary_search(&self.indexes, &query, &compare_func);
            insert_into_tree_node(&mut self.data[loc], new_data, query, compare_func);
        };
    }
}
