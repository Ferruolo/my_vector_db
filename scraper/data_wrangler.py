from tqdm import tqdm

with open('data/data.txt', 'r') as f:
    data = f.read()


def is_ascii(text):
    """Check if all characters in text are ASCII."""
    return all(ord(char) < 128 for char in text)


new_text = ""
for line in tqdm(data.split('\n')):
    for word in line.split(' '):
        if word:
            ascii_word = ''.join(char for char in word if ord(char) < 128)
            if ascii_word:  # Only add if there's something left after filtering
                new_text += ascii_word + ', 0;\n'

# Optional: Write to file
with open('data/processed_text.txt', 'w', encoding='ascii') as f:
    f.write(new_text)
