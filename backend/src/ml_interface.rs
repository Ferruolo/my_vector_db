use tokenizers::tokenizer::{Tokenizer};
use anyhow::{Result, Context};
use tch::{Tensor};

// Define a trait for tokenizer operations
pub trait TokenizerInterface {
    fn tokenize(&self, text: &str) -> Result<Vec<String>>;
    fn encode(&self, text: &str) -> Result<Vec<u32>>;
}

// Struct wrapper for the LLaMA tokenizer
pub struct LlamaTokenizer {
    tokenizer: Tokenizer,
}

impl LlamaTokenizer {
    pub(crate) fn new(model_path: &str) -> Result<Self> {
        let tokenizer = Tokenizer::from_file(model_path)?;
        Ok(LlamaTokenizer { tokenizer })
    }
}

// Implement the TokenizerInterface trait for LlamaTokenizer
impl TokenizerInterface for LlamaTokenizer {
    fn tokenize(&self, text: &str) -> Result<Vec<String>> {
        let encoding = self.tokenizer.encode(text, false)?;
        Ok(encoding.get_tokens().to_vec())
    }

    fn encode(&self, text: &str) -> Result<Vec<u32>> {
        let encoding = self.tokenizer.encode(text, false)?;
        Ok(encoding.get_ids().to_vec())
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