// File: src/main.rs

mod LlamaEmbedding;

use std::ffi::{CString, c_void};
use libc::{c_char, c_float};

#[link(name = "llamafile_embedding_lib")]
extern "C" {
    fn create_embedding(model_path: *const c_char) -> *mut c_void;
    fn destroy_embedding(embedding: *mut c_void);
    fn get_single_embedding(embedding: *mut c_void, text: *const c_char) -> *mut c_float;
    fn get_multiple_embeddings(embedding: *mut c_void, texts: *const *const c_char, num_texts: usize) -> *mut c_float;
}

struct LlamafileEmbedding {
    ptr: *mut c_void,
}

impl LlamafileEmbedding {
    fn new(model_path: &str) -> Self {
        let c_model_path = CString::new(model_path).unwrap();
        let ptr = unsafe { create_embedding(c_model_path.as_ptr()) };
        LlamafileEmbedding { ptr }
    }

    fn get_embedding(&self, text: &str) -> Vec<f32> {
        let c_text = CString::new(text).unwrap();
        let embedding_ptr = unsafe { get_single_embedding(self.ptr, c_text.as_ptr()) };
        unsafe { Vec::from_raw_parts(embedding_ptr, 768, 768) } // Assuming 768-dimensional embeddings
    }

    fn get_embeddings(&self, texts: &[String]) -> Vec<Vec<f32>> {
        let c_texts: Vec<*const c_char> = texts.iter()
            .map(|s| CString::new(s.as_str()).unwrap().into_raw())
            .collect();
        let embeddings_ptr = unsafe { get_multiple_embeddings(self.ptr, c_texts.as_ptr(), texts.len()) };
        let embeddings = unsafe { Vec::from_raw_parts(embeddings_ptr, texts.len() * 768, texts.len() * 768) };

        // Clean up CStrings
        for &ptr in &c_texts {
            unsafe { let _ = CString::from_raw(ptr as *mut c_char); }
        }

        embeddings.chunks(768).map(|chunk| chunk.to_vec()).collect()
    }
}

impl Drop for LlamafileEmbedding {
    fn drop(&mut self) {
        unsafe { destroy_embedding(self.ptr) };
    }
}

// Example Use

// fn main() {
//     let model = LlamafileEmbedding::new("/path/to/your/llamafile/model");
// 
//     let embedding = model.get_embedding("Hello, world!");
//     println!("Single embedding (first 5 values): {:?}", &embedding[..5]);
// 
//     let texts = vec!["Hello, world!".to_string(), "This is a test.".to_string()];
//     let embeddings = model.get_embeddings(&texts);
//     println!("Number of embeddings: {}", embeddings.len());
//     println!("First embedding (first 5 values): {:?}", &embeddings[0][..5]);
// }