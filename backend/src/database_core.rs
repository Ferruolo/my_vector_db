
// Interfaces
// This will make it easier to abstract as we
// Switch to more low level types


use tch::{Device, Kind, Tensor};

use crate::ml_interface::{Embedding, LlamaTokenizer, TokenizerInterface};

trait VectorItem {
    fn from_text(text_data: &String, embedding: &Embedding, llama_tokenizer: &LlamaTokenizer) -> Self;

    fn compare(&self, other: &Self) -> f32;
}

trait VectorDB  {
    fn add_item(&self, text_item: &String);

    fn find_k_neighbors(&self, text_item: &String, k: u8) -> Vec<String>;
}



struct DbItem {
    text_data: String, // TODO: make it a str?
    vector_data: Tensor
}


impl DbItem {
    fn new() -> DbItem {
        let vector_data = [0f32, 0f32, 0f32, 0f32, 0f32];
        Self {
            text_data: "".to_string(),
            vector_data: Box::new(*vector_data),
        }
    }
}

impl VectorItem for DbItem {

    fn from_text(text_data: &String, embedding: &Embedding, llama_tokenizer: &LlamaTokenizer) -> Self {
        let tokens = llama_tokenizer.encode(text_data).unwrap();
        let token_count = tokens.len() as f64;
        let sum_token = tokens.iter().fold(
            Tensor::zeros(&[4096], (Kind::Float, Device::Cpu)),
            |acc, &idx| acc + embedding.forward(idx as i64).unwrap()
        );
        let average_token = sum_token / token_count;
        Self {
            text_data: text_data.clone(),
            vector_data: average_token
        }
    }

    fn compare(&self, other: &DbItem) -> f32 {
        let dot_product = self.vector_data.dot(&other.vector_data);
        let norm_a = self.vector_data.norm();
        let norm_b = other.vector_data.norm();
        f32::try_from(dot_product / (norm_a * norm_b)).unwrap()
    }
}


struct VectorDBCore {
    data: Vec<DbItem>,
    tokenizer: LlamaTokenizer,
    embedding: Embedding
}




impl VectorDBCore {
    fn new(tokenizer_filepath: &String, embedding_filepath: &String) -> VectorDBCore {
        Self {
            data: vec![],
            tokenizer: match LlamaTokenizer::new(tokenizer_filepath)
            {
                Ok(x) => {x}
                Err(_) => {
                    panic!("Error Loading Llama Tokenizer from file")
                }
            },
            embedding: Embedding::load(embedding_filepath).unwrap(),
        }
    }
}


impl VectorDB for VectorDBCore {
    fn add_item(&mut self, text_item: &String) {
        let tokenized_item = match self.tokenizer.encode(text_item) {
            Ok(x) => {x}
            Err(_) => {
                panic!("Issue tokenizing item {}", text_item);
            }
        };
        self.data.push(DbItem::from_text(text_item, &self.embedding, &self.tokenizer));
    }

    fn find_k_neighbors(&self, text_item: &String, k: u8) {
        let query_item = DbItem::from_text(text_item, &self.embedding, &self.tokenizer);
        let mut scores: Vec<(usize, f32)> = vec![];
        scores.reserve(self.data.len());
        for i in 0..self.data.len() {
            scores.push((i, self.data[i].compare(&query_item)));
        }
        scores.sort_by(|a, b| {b[1].cmp(a[1])});

        let text_pieces = scores[0..k].map(|x|{self.data[x].text_data});
        text_pieces
    }
}




