use tokenizers::tokenizer::{Tokenizer};
use tch::nn::Module;
use anyhow::{Result, Context};
use tch::{Device, Tensor, CModule};

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
    module: CModule,
}

impl Embedding {
    pub fn load(path: &str) -> Result<Self> {
        let device = Device::Cpu;
        let module = CModule::load(path)
            .with_context(|| format!("Failed to load model from {}", path))?;
        Ok(Self { module })
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        self.module.forward(input)
    }
}