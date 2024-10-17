use crate::llama_embedding::LlamafileEmbedding;
use crate::node_interface::NodeInterface;
use tch::Tensor;

pub(crate) struct Node<T> {
    data: Vec<T>,
    indexes: Vec<Tensor>,
}

pub(crate) enum TreeNode<T> {
    LeafNode(Node<T>),
    Null,
    OverflowNode(Box<TreeNode<T>>, Tensor, Box<TreeNode<T>>),
}

pub(crate) struct VectorDB<T> {
    data: Vec<TreeNode<T>>,
    indexes: Vec<Tensor>,
    embedding_item: LlamafileEmbedding,
    zero: Tensor,
}

pub(crate) enum ChildType<T> {
    Data(T),
    Node(TreeNode<T>),
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

    fn push_back(&mut self, index: Tensor, datum: ChildType<T>, loc: usize) {
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
        unimplemented!()
    }

    fn push(&mut self, index: Tensor, datum: ChildType<T>) {
        unimplemented!()
    }

    fn push_last_element(&mut self) {
        unimplemented!()
    }

    fn pop_usize_max(&mut self) {
        unimplemented!()
    }

    fn move_data_to(&mut self, other: Box<Self>) {
        unimplemented!()
    }
}
