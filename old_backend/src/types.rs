use serde::{Deserialize, Serialize};
use crate::node_interface::NodeInterface;
use tch::Tensor;

pub(crate) struct Node<T> {
    pub data: Vec<T>,
    pub indexes: Vec<Tensor>,
}

pub(crate) enum TreeNode<T> {
    LeafNode(Node<T>),
    Null,
    OverflowNode(Box<TreeNode<T>>, Tensor, Box<TreeNode<T>>),
}

pub(crate) enum ChildType<T> {
    Data(T),
    Node(TreeNode<T>),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Response {
    Success,
    Error(String),
    Data(Vec<String>),
    Indexes(Vec<usize>)
}

#[derive(Debug, Serialize, Deserialize)]
pub (crate) struct InsertRequest {
    pub(crate) entry: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub (crate) struct GetRequest {
    query: String,
}

pub (crate) enum Request {
    Insert { id: String, text: String },
    Get { id: String },
    Shutdown,
}

impl<T> NodeInterface<T> for Node<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            indexes: Vec::new(),
        }
    }

    fn reverse_data(&mut self) {
        self.data.reverse();
        self.indexes.reverse();
    }

    fn pop_last_data_and_index(&mut self) -> Option<(Tensor, ChildType<T>)> {
        match (self.indexes.pop(), self.data.pop()) {
            (Some(index), Some(data)) => Some((index, ChildType::Data(data))),
            (_, _) => None,
        }
    }

    fn push_back(&mut self, index: Tensor, datum: ChildType<T>) {
        match datum {
            ChildType::Data(x) => self.data.push(x),
            _ => panic!("Tried to insert non data type into data"),
        }
        self.indexes.push(index);
    }

    fn get_loc(&self, index: &Tensor) -> usize {
        unimplemented!()
    }

    fn get_midpoint_idx(&self) -> usize {
        return self.data.len().div_ceil(2);
    }

    fn get_index_len(&self) -> usize {
        return self.indexes.len();
    }

    fn create_new_with_data(index: Tensor, data: ChildType<T>) -> Self {
        match data {
            ChildType::Data(x) => Self {
                data: vec![x],
                indexes: vec![index],
            },
            _ => panic!("Tried to create node with non-data type"),
        }
    }
}
