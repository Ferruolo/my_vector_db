use crate::llama_embedding::LlamafileEmbedding;
use crate::vector_db::TreeNode::{LeafNode, Null};
use libc::PARMRK;
use tch::kind::{FLOAT_CPU, FLOAT_CUDA};
use tch::Kind::Float;
use tch::Tensor;
use crate::helpers::{binary_search, cosine_similarity_rust_float};

const ELEMENTS_PER_PAGE: usize = 10;

Node <T> {
    indexes: Vector<Tensor>,
    data: Vector<T>,
}

enum TreeNode {
    Null
    LeafNode(Node)
}


struct VectorDB <T> {
    zero: Tensor,
    data: Vector<TreeNode>,
    embedding_item: LlamafileEmbedding
}



impl VectorDB<T> {
    pub fn new(model_path: &str, dims: usize) -> Self {
        let zero = Tensor::zeros(dims, FLOAT_CUDA);
        let embedding_model = LlamafileEmbedding::new(model_path);
        Self {
           zero: zero,
           data: vec![],
           embedding_item: embedding_model
        }
    }

    pub fn insert(&mut self, new_data: T) {
       if self.data.isEmpty() {

       } else {
          let compare = |l: &Tensor, r: &Tensor|{
            cosine_similarity_rust_float(l, &self.zero) < cosine_similarity_rust_float(r, &self.zero)
          };
          let loc = binary_search(indexes, query, compare);


       }
    }

}
