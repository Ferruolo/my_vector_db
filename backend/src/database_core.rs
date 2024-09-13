use tch::{Device, Kind, Tensor};
use crate::ml_interface::{Embedding, LlamaTokenizer, TokenizerInterface};

trait VectorItem {
    fn from_text(text_data: &str, embedding: &Embedding, llama_tokenizer: &LlamaTokenizer) -> Self;
    fn compare(&self, other: &Self) -> f32;
}

fn compare_f32(a: f32, b: f32) -> std::cmp::Ordering {
    a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
}

struct DbItem {
    text_data: String,
    vector_data: Tensor
}

impl DbItem {
    fn new() -> DbItem {
        Self {
            text_data: String::new(),
            vector_data: Tensor::zeros(&[4096], (Kind::Float, Device::Cpu)),
        }
    }
}

impl VectorItem for DbItem {
    fn from_text(text_data: &str, embedding: &Embedding, llama_tokenizer: &LlamaTokenizer) -> Self {
        let tokens = llama_tokenizer.encode(text_data).unwrap();
        let token_count = tokens.len() as f64;
        let sum_token = tokens.iter().fold(
            Tensor::zeros(&[4096], (Kind::Float, Device::Cpu)),
            |acc, &idx| acc + embedding.forward(idx as i64).unwrap()
        );
        let average_token = sum_token / token_count;
        Self {
            text_data: text_data.to_string(),
            vector_data: average_token
        }
    }

    fn compare(&self, other: &DbItem) -> f32 {
        let dot_product = self.vector_data.dot(&other.vector_data);
        let norm_a = self.vector_data.norm();
        let norm_b = other.vector_data.norm();
        (dot_product / (norm_a * norm_b)).try_into().unwrap()
    }
}

pub struct VectorDBCore {
    data: Vec<DbItem>,
    tokenizer: LlamaTokenizer,
    embedding: Embedding
}

impl VectorDBCore {
    pub fn new(tokenizer_filepath: &str, embedding_filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            data: Vec::new(),
            tokenizer: LlamaTokenizer::new(tokenizer_filepath),
            embedding: Embedding::load(embedding_filepath)?,
        })
    }

    pub fn add_item(&mut self, text_item: &str) {
        self.data.push(DbItem::from_text(text_item, &self.embedding, &self.tokenizer));
    }

    pub fn find_k_neighbors(&self, text_item: &str, k: usize) -> Vec<String> {
        let query_item = DbItem::from_text(text_item, &self.embedding, &self.tokenizer);
        let mut scores: Vec<(usize, f32)> = self.data.iter().enumerate()
            .map(|(i, item)| (i, item.compare(&query_item)))
            .collect();

        scores.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        scores.into_iter()
            .take(k)
            .map(|(i, _)| self.data[i].text_data.clone())
            .collect()
    }
}