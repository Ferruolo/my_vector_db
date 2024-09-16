use anyhow::{Context, Result};
use tch::{jit, Tensor};
use tiktoken_rs::{p50k_base, CoreBPE};
use crate::EMBEDDING_PATH;

// Define a trait for tokenizer operations
pub trait TokenizerInterface {
    fn encode(&self, text: &str) -> Vec<usize>;
}

// Struct wrapper for the LLaMA tokenizer
pub struct LlamaTokenizer {
    tokenizer: CoreBPE,
}

impl LlamaTokenizer {
    pub(crate) fn new() -> Self {
        let tokenizer  = match p50k_base() {
            Ok(x) => {x}
            Err(err) => {panic!("Error: {}", err)}
        };
        Self { tokenizer }
    }
}


// Implement the TokenizerInterface trait for LlamaTokenizer
impl TokenizerInterface for LlamaTokenizer {
    fn encode(&self, text: &str) -> Vec<usize> {
        self.tokenizer.encode_with_special_tokens(text)
    }
}

pub struct Embedding {
    weights: Tensor,
    input_dim: i64,
    output_dim: i64,
}

impl Embedding {
    pub fn load(path: &str) -> Result<Self> {
        let model =jit::CModule::load(path).with_context(|| "Failed to load model")?;
        let weights: Tensor = match model.named_parameters() {
            Ok(params) => {
                params
                    .iter()
                    .find(|(name, _)| name == "lay_1.weight")
                    .map(|(_, tensor)| tensor.detach())
                    .unwrap_or_else(|| panic!("Weight 'lay_1.weight' not found"))
            }
            Err(err) => panic!("Error: {}", err),
        };
        let shape = weights.size();
        if shape.len() != 2 {
            anyhow::bail!("Expected 2D weight tensor, got {:?}D", shape.len());
        }

        let input_dim = shape[0];
        let output_dim = shape[1];

        println!("Dimensions: ({}, {})", input_dim, output_dim);
        Ok(Self { weights, input_dim, output_dim })
    }

    pub fn forward(&self, input_index: i64) -> Result<Tensor> {
        if input_index < 0 || input_index >= self.output_dim {
            anyhow::bail!("Invalid input index: {}. Expected 0 <= index < {}", input_index, self.input_dim);
        }

        let output = self.weights.select(1, input_index);
        Ok(output)
    }

    pub fn input_dim(&self) -> i64 {
        self.input_dim
    }

    pub fn output_dim(&self) -> i64 {
        self.output_dim
    }
}