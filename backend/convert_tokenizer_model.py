from transformers import PreTrainedTokenizerFast
import json
import os


def convert_tokenizer_model_to_json(model_path, json_path):
    try:
        # Check if the model file exists
        if not os.path.exists(model_path):
            raise FileNotFoundError(f"The file {model_path} does not exist.")

        # Load the tokenizer from the .model file
        tokenizer = PreTrainedTokenizerFast(tokenizer_file=model_path)

        # Get the tokenizer config as a dictionary
        tokenizer_config = tokenizer.to_dict()

        # Save the tokenizer config as a JSON file
        with open(json_path, 'w', encoding='utf-8') as f:
            json.dump(tokenizer_config, f, ensure_ascii=False, indent=2)

        print(f"Tokenizer config saved to {json_path}")

    except json.JSONDecodeError:
        print(f"Error: The file {model_path} is not a valid JSON file.")
    except Exception as e:
        print(f"An error occurred: {str(e)}")
        print("Please ensure that:")
        print("1. The input file is a valid tokenizer.model file.")
        print("2. You have the correct version of the transformers library installed.")
        print("3. You have read permissions for the input file and write permissions for the output directory.")

# Example usage:
# convert_tokenizer_model_to_json('path/to/tokenizer.model', 'paA
# th/to/tokenizer.json')

convert_tokenizer_model_to_json("./Meta-Llama-3.1-8B/tokenizer.model", "./Meta-Llama-3.1-8B/tokenizer.json")