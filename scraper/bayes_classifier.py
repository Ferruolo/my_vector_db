from transformers import GPT2Tokenizer
from shared.helpers import drop_repeated_newline_regex
# Load the tokenizer
tokenizer = GPT2Tokenizer.from_pretrained("gpt2")

# Read a file
with open('data.txt', 'r', encoding='utf-8') as file:
    text = file.read()
text = drop_repeated_newline_regex(text)



# Tokenize
encoded = tokenizer.encode(text)  # Returns token IDs

