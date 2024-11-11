from llama_index.embeddings.llamafile import LlamafileEmbedding
from typing import List
import numpy as np


class LlamafileEmbeddingInterface:
    def __init__(self, model_path: str):
        self.embedding_model = LlamafileEmbedding(model_path=model_path)

    def get_embedding(self, text: str) -> List[float]:
        embedding = self.embedding_model.get_text_embedding(text)
        return embedding.tolist()

    def get_embeddings(self, texts: List[str]) -> List[List[float]]:
        embeddings = self.embedding_model.get_text_embeddings(texts)
        return [emb.tolist() for emb in embeddings]


# Example usage
if __name__ == "__main__":
    model_path = "/path/to/your/llamafile/model"
    interface = LlamafileEmbeddingInterface(model_path)

    # Single embedding
    text = "Hello, world!"
    embedding = interface.get_embedding(text)
    print(f"Embedding for '{text}': {embedding[:5]}...")  # Print first 5 elements

    # Multiple embeddings
    texts = ["Hello, world!", "This is a test.", "LlamaIndex is cool!"]
    embeddings = interface.get_embeddings(texts)
    print(f"Number of embeddings: {len(embeddings)}")
    print(f"First embedding: {embeddings[0][:5]}...")  # Print first 5 elements of first embedding
