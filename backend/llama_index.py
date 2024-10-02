from llama_index.embeddings.llamafile import LlamafileEmbedding

embedding = LlamafileEmbedding(
    base_url="http://127.0.0.1:8080",
)

pass_embedding = embedding.get_text_embedding_batch(
    ["This is a passage!", "This is another passage"], show_progress=True
)
print(len(pass_embedding), len(pass_embedding[0]))

query_embedding = embedding.get_query_embedding("This is a string. There are many like it, but this one is mine")
print(len(query_embedding))
print(query_embedding)