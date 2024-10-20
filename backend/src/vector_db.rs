use crate::helpers::{binary_search, binary_search_floats, compare, cosine_similarity_rust_float};
use crate::llama_embedding::LlamafileEmbedding;
use crate::node_interface::NodeInterface;
use crate::types::*;
use crate::vector_db::TreeNode::{LeafNode, Null};
use tch::{Device, Tensor};

const ELEMENTS_PER_PAGE: usize = 10;

pub(crate) struct VectorDB<T: Clone> {
    data: Vec<TreeNode<T>>,
    indexes: Vec<Tensor>,
    embedding_item: LlamafileEmbedding,
    zero: Tensor,
}

impl<T: Clone> VectorDB<T> {
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
            insert_into_tree_node(self.data[loc], new_data, query, compare_func);
        };
    }

    pub fn get_top_k_indexes(&self, query_string: String, k: usize) -> Vec<usize> {
        let query = self.embedding_item.get_embedding(query_string.as_str());
        let mut index_vec = vec![];
        let mut dist_vec: Vec<f32> = vec![];
        let mut idx = 0;
        for node in self.data {
            match node {
                LeafNode(node) => {
                    let node_item: Node<T> = node.into();
                    for i in 0..node_item.get_index_len() {
                        let dist = cosine_similarity_rust_float(&query, &node_item.indexes[i]);
                        let loc = binary_search_floats(&dist_vec, &dist);
                        index_vec.insert(loc, idx);
                        dist_vec.insert(loc, dist);
                        if dist_vec.len() > k {
                            dist_vec.pop();
                            index_vec.pop();
                        }
                        idx += 1;
                    }
                }
                _ => {}
            }
        }
        index_vec
    }

    pub fn get_indexes(&self, indices: Vec<usize>) -> Vec<String> {
        let mut index_data: Vec<String> = Vec::new();
        for index in indices {
            let node_index = index / ELEMENTS_PER_PAGE;
            let item_index = index % ELEMENTS_PER_PAGE;
            let node = match self.data.get(node_index).unwrap() {
                LeafNode(x) => {x}
                _ => {panic!("Node invalid")}
            };
            let new_data= match node.data.get(item_index) {
                None => {panic!("Invalid Index Provided")}
                Some(datum) => {datum.clone()}
            };
            index_data.push(new_data);
        }
        index_data
    }

    pub fn save(&self) {
        println!("You need to implement save you idiot!");
    }
}

fn split_node<T>(mut node: Node<T>) -> TreeNode<T> {
    if node.get_index_len() < ELEMENTS_PER_PAGE {
        return LeafNode(node);
    }
    let midpt = node.get_midpoint_idx();
    node.reverse_data();
    let (mut left, mut right) = (Node::new(), Node::new());
    let mut selected: &mut Node<T> = &mut right;
    while let Some((idx, datum)) = node.pop_last_data_and_index() {
        if node.get_index_len() < midpt {
            selected = &mut left;
        }
        selected.push_back(idx, datum);
    }
    let new_index = right.indexes.first().unwrap().copy();
    TreeNode::OverflowNode(Box::new(LeafNode(left)), new_index, Box::new(LeafNode(right)))
}

fn insert_into_tree_node<T>(
    node: TreeNode<T>,
    new_data: T,
    query: Tensor,
    compare: impl Fn(&Tensor, &Tensor) -> bool,
) -> TreeNode<T> {
    match node {
        LeafNode(mut node) => {
            let loc = binary_search(&node.indexes, &query, &compare);
            node.data.insert(loc, new_data);
            node.indexes.insert(loc, query);
            split_node(node)
        }
        Null => {
            todo!();
        }
        TreeNode::OverflowNode(_, _, _) => {
            panic!("Should never be inserting into an overflow node")
        }
    }
}
