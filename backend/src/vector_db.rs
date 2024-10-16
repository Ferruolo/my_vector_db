use crate::llama_embedding::LlamafileEmbedding;
use tch::Kind::Float;
use tch::kind::{FLOAT_CPU, FLOAT_CUDA};
use tch::Tensor;

const ELEMENTS_PER_PAGE: usize = 4;
type IndexType = Tensor;


struct DataType {
    index: IndexType,
    data: String
}

impl DataType {
    fn new(index: IndexType, data: String) -> DataType {
        Self { index, data }
    }
}



enum ChildType {
    Data(DataType),
    Child(Node)
}

struct Node {
    index: Vec<IndexType>,
    children: Vec<ChildType>,
}

enum TreeNode {
    Null,
    InternalNode,
    LeafNode(Node),
    OverflowNode,
}


struct VectorDB {
    root_node: TreeNode,
    embedding: LlamafileEmbedding
}

impl VectorDB {
    fn new(llamafile_embedding: LlamafileEmbedding) -> Self {
        Self {
            root_node: TreeNode::Null,
            embedding: llamafile_embedding,
        }
    }

    fn insert_data(&mut self, data: String) -> TreeNode {
        let new_vec = self.embedding.get_embedding(data.as_str());
        self.root_node = insert(self.root_node, DataType::new(new_vec, data));
    }

}

fn compare(l: &Tensor, r: &Tensor) -> bool {
    l.dot(r).sum(Float).int64_value(&[]) > 0
}

fn search(index: &Vec<Tensor>, query: &Tensor) -> usize {
    let mut low: usize = 0;
    let mut high: usize = index.len();

    while low < high {
        let mid = (low + high) / 2;
        if compare(&index[mid], query) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    if compare(&index[low], query) {
        low
    } else {
        low + 1
    }
}


fn define_split_vector(vectors: &Vec<Tensor>) -> Tensor {
    let num_dim = vectors.first().unwrap().size()[0];
    let zero = Tensor::zeros(&[num_dim], (FLOAT_CUDA));
    let mut indexes = vectors.iter().enumerate().map(|(i, x)| {i}).collect::<Vec<usize>>();
    indexes.sort_by(|l, r| {(l - zero).partial_comp(r- zero)}); // Sort by distance from zero
    let new_tensor = indexes.iter().enumerate().map(|(i, x)| {
        if i < indexes.len() / 2 {
            - vectors[*x].copy()
        } else {
            vectors[*x].copy()
        }
    }).reduce(|prev,x|{prev+x}).unwrap();
    
    new_tensor
}



fn insert(node: TreeNode, data: DataType) -> TreeNode {
    match node {
        TreeNode::Null => {}
        TreeNode::LeafNode(node) => {
            if node.children.len() > 1 {
                
            } else {

            }

            let loc = search(&node.index, &data.index);
        }
        _ => todo!();
    }



}