
// Interfaces
// This will make it easier to abstract as we
// Switch to more low level types

use tokenizers::Tokenizer;
use crate::ml_interface::{Embedding, LlamaTokenizer, TokenizerInterface};

trait VectorItem {
    fn from_text(text_data: &String) -> Self;

    fn compare(&self, other: &Self) -> f32;
}

trait VectorDB  {
    fn add_item(&self, text_item: &String);

    fn find_k_neighbors(&self, text_item: &String, k: u8);
}



struct DbItem {
    text_data: String, // TODO: make it a str?
    vector_data: Box([f32])
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

    fn from_text(text_data: &String) -> Self {
        let vector_data = [0f32, 0f32, 0f32, 0f32, 0f32];
        Self {
            text_data: text_data.clone(),
            vector_data: Box::new(*vector_data)
        }
    }

    fn compare(&self, other: &DbItem) -> f32 {
        0.0
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
        self.data.push(DbItem::from_text(text_item));
    }

    fn find_k_neighbors(&self, text_item: &String, k: u8) {
        todo!()
    }
}




