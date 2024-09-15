use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use anyhow::{Result, Context};
use serde_json::Value;
use tch::Tensor;
use tiktoken_rs::CoreBPE;
use rustc_hash::FxHashMap;


// Define a trait for tokenizer operations
pub trait TokenizerInterface {
    fn encode(&self, text: &str) -> Result<Vec<usize>>;
}

// Struct wrapper for the LLaMA tokenizer
pub struct LlamaTokenizer {
    tokenizer: CoreBPE,
}

impl LlamaTokenizer {
    pub(crate) fn new(vocab_file: &str) -> Result<Self> {
        let mut file = File::open(vocab_file)
            .with_context(|| format!("Failed to open vocab file: {}", vocab_file))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .with_context(|| format!("Failed to read vocab file: {}", vocab_file))?;

        let json: Value = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse JSON from vocab file: {}", vocab_file))?;

        let encoder: FxHashMap<Vec<u8>, usize> = json["model"]["vocab"]
            .as_object()
            .context("Failed to get vocab object")?
            .iter()
            .map(|(k, v)| (k.as_bytes().to_vec(), v.as_u64().unwrap() as usize))
            .collect();

        let special_tokens: FxHashMap<Vec<u8>, usize> = json["model"]["special_tokens"]
            .as_object()
            .context("Failed to get special tokens object")?
            .iter()
            .map(|(k, v)| (k.as_bytes().to_vec(), v.as_u64().unwrap() as usize))
            .collect();

        let merges: Vec<(Vec<u8>, Vec<u8>)> = json["model"]["merges"]
            .as_array()
            .context("Failed to get merges array")?
            .iter()
            .map(|v| {
                let parts: Vec<&str> = v.as_str().unwrap().split_whitespace().collect();
                (parts[0].as_bytes().to_vec(), parts[1].as_bytes().to_vec())
            })
            .collect();

        let tokenizer = CoreBPE::new(encoder, special_tokens, merges)
            .context("Failed to create CoreBPE tokenizer")?;

        Ok(LlamaTokenizer { tokenizer })

    }
}


// Implement the TokenizerInterface trait for LlamaTokenizer
impl TokenizerInterface for LlamaTokenizer {
    fn encode(&self, text: &str) -> Result<Vec<usize>> {
        Ok(self.tokenizer.encode_ordinary(text))
    }
}

pub struct Embedding {
    weights: Tensor,
    input_dim: i64,
    output_dim: i64,
}

impl Embedding {
    pub fn load(path: &str) -> Result<Self> {
        let weights = Tensor::load(path)
            .with_context(|| format!("Failed to load weights from {}", path))?;

        let shape = weights.size();
        if shape.len() != 2 {
            anyhow::bail!("Expected 2D weight tensor, got {:?}D", shape.len());
        }

        let input_dim = shape[0];
        let output_dim = shape[1];

        Ok(Self { weights, input_dim, output_dim })
    }

    pub fn forward(&self, input_index: i64) -> Result<Tensor> {
        if input_index < 0 || input_index >= self.input_dim {
            anyhow::bail!("Invalid input index: {}. Expected 0 <= index < {}", input_index, self.input_dim);
        }

        let output = self.weights.select(0, input_index);
        Ok(output)
    }

    pub fn input_dim(&self) -> i64 {
        self.input_dim
    }

    pub fn output_dim(&self) -> i64 {
        self.output_dim
    }
}