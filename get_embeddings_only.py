import torch
from torch.nn.modules.module import T

device_name = 'cuda' if torch.cuda.is_available() else 'mps'

data = torch.load("./old_backend/Meta-Llama-3.1-8B/consolidated.00.pth", weights_only=False, map_location=torch.device('cpu'))

class Embedding(torch.nn.Module):
    def __init__(self):
        super(Embedding, self).__init__()
        self.lay_1 = torch.nn.Linear(128256, 4096, dtype=torch.float32)
    def forward(self, x):
        return self.lay_1(x)

embedding = Embedding()
embedding.lay_1.weight.data = data['tok_embeddings.weight'].data.clone().detach().to(dtype=torch.float32).T
del data
# Create a sample input tensor
sample_input = torch.randn(1, 128256).to(dtype=torch.float32)

# Use torch.jit.trace to create a TorchScript model
traced_model = torch.jit.trace(embedding, sample_input)

# Save the traced model
torch.jit.save(traced_model, './old_backend/embedding.pt')

