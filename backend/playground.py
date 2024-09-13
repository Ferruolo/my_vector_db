import torch
from torch.nn.modules.module import T

data = torch.load("Meta-Llama-3.1-8B/consolidated.00.pth", weights_only=False, map_location=torch.device('mps'))


class Embedding(torch.nn.Module):
    def __init__(self):
        super(Embedding, self).__init__()
        self.lay_1 = torch.nn.Linear(128256, 4096)
    def forward(self, x):
        return self.lay_1(x)


embedding = Embedding()
embedding.lay_1.weight.data = data['tok_embeddings.weight'].data

torch.save(embedding, 'embedding.pth')