import re
from dotenv import load_dotenv
import json
import base64
import requests
from typing import Optional, List, Union
from anthropic import Anthropic
from llama_index.core.node_parser import SentenceSplitter
from llama_index.core.text_splitter import TokenTextSplitter

load_dotenv()


class LLMWrapper:
    def __init__(self, url="", api_key=None):
        self.url = url
        self.api_key = api_key

    def _encode_image(self, image_path: str):
        with open(image_path, "rb") as f:
            img = f.read()
        img_base64 = base64.b64encode(img)
        return img_base64

    def _get_media_type(self, image_path: str) -> str:
        extension = re.search(r'\.(\w+)$', image_path).group(1).lower()
        return f"{extension}"

    def make_call(self, prompt: str, system_prompt: Optional[str] = None,
                  image_paths: Optional[List[str]] = None) -> str:
        pass


class ClaudeWrapper(LLMWrapper):
    def __init__(self, url="", api_key=None, model_name="claude-3-sonnet-20240229"):
        super().__init__(url, api_key)
        self.client = Anthropic(api_key=self.api_key)
        self.model_name = model_name

    def make_call(self, prompt: str, system_prompt: Optional[str] = None, image_paths: Optional[List[str]] = None,
                  file_data: Optional[bytes] = None, file_type: Optional[str] = None) -> str:
        content = []

        if file_data and file_type:
            encoded_data = base64.b64encode(file_data).decode()
            content.append(
                {"type": "file", "source": {"type": "base64", "media_type": file_type, "data": encoded_data}})

        if image_paths:
            for path in image_paths:
                image_data = self._encode_image(path)
                content.append({"type": "image",
                    "source": {"type": "base64", "media_type": f"image/{self._get_media_type(path)}",
                        "data": image_data.decode()}})

        content.append({"type": "text", "text": prompt})

        message = self.client.messages.create(model=self.model_name, max_tokens=1024,
            messages=[{"role": "user", "content": content}], system=system_prompt if system_prompt else "")

        try:
            return message.content[0].text
        except (KeyError, IndexError, AttributeError):
            return ""

    def extract_pdf_data(self, pdf_link: str) -> dict:
        if pdf_link.startswith(('http://', 'https://')):
            response = requests.get(pdf_link)
            pdf_data = response.content
        else:
            with open(pdf_link, 'rb') as f:
                pdf_data = f.read()

        prompt = """Extract all relevant information from this PDF and return it as a JSON object with the following structure:
        {
            "title": "document title",
            "date": "document date if present",
            "author": "document author if present",
            "content": {
                "key_points": [],
                "sections": {},
                "tables": [],
                "metadata": {}
            }
        }"""

        response = self.make_call(prompt=prompt, file_data=pdf_data, file_type="application/pdf")

        try:
            return json.loads(response)
        except json.JSONDecodeError:
            return {"error": "Failed to parse JSON response", "raw_response": response}

    def extract_menu_data(self, data: str) -> dict:
        prompt = """Analyze this menu text and return a JSON object with this exact structure:
        {
            "categories": [
                {
                    "name": "category name",
                    "items": [
                        {
                            "name": "item name",
                            "price": "price as decimal",
                            "description": "item description",
                            "dietary_info": ["dietary tags"]
                        }
                    ]
                }
            ]
        }"""

        response = self.make_call(prompt.format(data=data))

        try:
            return json.loads(response)
        except json.JSONDecodeError:
            return {"error": "Failed to parse JSON response", "raw_response": response}

    def extract_locations(self, data: str) -> List[dict]:
        prompt = """Extract all locations from this text and return them as a JSON array with this structure:
        [
            {
                "full_address": "complete address string",
                "components": {
                    "street": "street address",
                    "city": "city name",
                    "state": "state/province",
                    "country": "country name",
                    "postal_code": "postal/zip code"
                },
                "context": "any additional location context"
            }
        ]"""

        response = self.make_call(prompt)

        try:
            return json.loads(response)
        except json.JSONDecodeError:
            return [{"error": "Failed to parse JSON response", "raw_response": response}]

    def get_embeddings(self, data: str) -> List[List[float]]:
        sentence_splitter = SentenceSplitter(chunk_size=1024, chunk_overlap=200, paragraph_separator="\n\n",
            tokenizer=TokenTextSplitter())

        chunks = sentence_splitter.split_text(data)
        embeddings = []

        for chunk in chunks:
            try:
                embedding_response = self.client.embeddings.create(model="claude-3-embedding-20240229",
                    input=chunk.text if hasattr(chunk, 'text') else chunk)
                embeddings.append(embedding_response.embeddings[0])
            except Exception as e:
                print(f"Error generating embedding: {str(e)}")
                continue

        return embeddings


class LlavaWrapper(LLMWrapper):
    def __init__(self, url="", api_key=None):
        super().__init__(url, api_key)
        self.url = url if url else "http://localhost:8080/"

    def make_call(self, prompt: str, system_prompt: Optional[str] = None,
                  image_paths: Optional[List[str]] = None) -> str:
        payload = {"model": "llava", "prompt": prompt, "system": system_prompt if system_prompt else ""}

        if image_paths:
            payload["images"] = []
            for path in image_paths:
                image_data = self._encode_image(path)
                payload["images"].append(
                    {"image": image_data.decode(), "media_type": f"image/{self._get_media_type(path)}"})

        response = requests.post(f"{self.url}/completion", json=payload)
        if response.status_code != 200:
            raise Exception(f"API call failed with status code: {response.status_code}")

        result = response.json()
        return result


class LlamafileWrapper:
    def __init__(self, base_url: str = "http://localhost:8080"):
        self.base_url = base_url.rstrip('/')

    def completion(self, prompt: str) -> str:
        payload = {"prompt": prompt, "stream": False, "temperature": 0.7, "top_p": 0.9, "max_tokens": 4096}

        response = requests.post(f"{self.base_url}/completion", json=payload)
        response.raise_for_status()
        return response.json()

    def embedding(self, text: Union[str, List[str]], dims: int = 4096) -> Union[List[float], List[List[float]]]:
        if isinstance(text, str):
            payload = {"text": text, "dims": dims}
            response = requests.post(f"{self.base_url}/embedding", json=payload)
            response.raise_for_status()
            return response.json()["embedding"]
        else:
            return [self.embedding(t, dims) for t in text]
