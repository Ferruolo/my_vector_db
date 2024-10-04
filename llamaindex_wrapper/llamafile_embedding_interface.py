
from llama_index.embeddings.llamafile import LlamafileEmbedding
import torch
import numpy as np

class LlamafileEmbeddingInterface:
    def __init__(self, model_path: str):
        self.embedding_model = LlamafileEmbedding(model_path=model_path)

    def get_embedding(self, text: str) -> np.ndarray:
        embedding = self.embedding_model.get_text_embedding(text)
        return np.array(embedding, dtype=np.float32)

    def get_embeddings(self, texts: list) -> np.ndarray:
        embeddings = self.embedding_model.get_text_embeddings(texts)
        return np.array(embeddings, dtype=np.float32)

def init(model_path: str) -> LlamafileEmbeddingInterface:
    return LlamafileEmbeddingInterface(model_path)

