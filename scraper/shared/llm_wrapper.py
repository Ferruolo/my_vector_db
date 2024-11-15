import base64
import json
import os
import re
from typing import Optional, List, Tuple

import requests
from anthropic import Anthropic
from dotenv import load_dotenv
from llama_index.core.text_splitter import TokenTextSplitter

from shared.helpers import retry_with_backoff, extract_json
from shared.models import Restaurant
from shared.prompts import format_extract_structured_data, format_extract_PDF

load_dotenv()


class ClaudeFailureError(Exception):
    pass


def _get_media_type(image_path: str) -> str:
    extension = re.search(r'\.(\w+)$', image_path).group(1).lower()
    return f"{extension}"


def get_embedding(text, api_key):
    response = requests.post(
        "https://api.voyageai.com/v1/embeddings",
        headers={"Authorization": f"Bearer {api_key}"},
        json={"model": "voyage-3", "input": text}
    )
    return response.json()["data"][0]["embedding"]


class LLMWrapper:
    def __init__(self, url="", api_key=None):
        self.url = url
        self.api_key = api_key

    def _encode_image(self, image_path: str):
        with open(image_path, "rb") as f:
            img = f.read()
        img_base64 = base64.b64encode(img)
        return img_base64

    def make_call(self, prompt: str, system_prompt: Optional[str] = None,
                  image_paths: Optional[List[str]] = None) -> str:
        pass


class ClaudeWrapper(LLMWrapper):
    def __init__(self, url="", api_key=None, model_name="claude-3-sonnet-20240229"):
        super().__init__(url, api_key)
        self.client = Anthropic(api_key=os.environ.get("CLAUDE_API_KEY"))
        self.model_name = model_name
        self.voyage_api_key = os.environ.get("VOYAGE_API_KEY")

    def make_call(self, prompt: str, system_prompt: Optional[str] = None, image_paths: Optional[List[str]] = None,
                  file_data: Optional[bytes] = None, file_type: Optional[str] = None, max_retries=10) -> str:
        content = []

        if file_data and file_type:
            encoded_data = base64.b64encode(file_data).decode()
            content.append(
                {"type": "file", "source": {"type": "base64", "media_type": file_type, "data": encoded_data}})

        if image_paths:
            for path in image_paths:
                image_data = self._encode_image(path)
                content.append({"type": "image",
                                "source": {"type": "base64", "media_type": f"image/{_get_media_type(path)}",
                                           "data": image_data.decode()}})

        content.append({"type": "text", "text": prompt})

        @retry_with_backoff()
        def get_message():
            message = self.client.messages.create(model=self.model_name,
                                                  max_tokens=4096,
                                                  messages=[{"role": "user", "content": content}])
            return message

        result = get_message()
        try:
            return result.content[0].text
        except (KeyError, IndexError, AttributeError):
            raise ClaudeFailureError()

    def extract_pdf_data(self, pdf_link: str) -> dict:
        response = requests.get(pdf_link)
        pdf_data = response.content

        prompt = format_extract_PDF(pdf_data).decode("utf-8")
        response = self.make_call(prompt=prompt)
        try:
            return json.loads(response)
        except json.JSONDecodeError:
            return {"error": "Failed to parse JSON response", "raw_response": response}

    def extract_structured_data(self, data: str, max_retries=3) -> Restaurant:
        for i in range(max_retries):
            response = self.make_call(format_extract_structured_data(data=data))

            with open("claude_response.json", 'w') as f:
                f.write(response)
            try:
                response = extract_json(response)
                with open("claude_response_clean.json", 'w') as f:
                    f.write(json.dumps(response))
                return Restaurant(**response)
            except json.JSONDecodeError:
                print("Failed (retrying")
        raise ClaudeFailureError()

    def get_embeddings(self, data: str) -> List[Tuple[str, List[float]]]:
        text_splitter = TokenTextSplitter(chunk_size=1024, chunk_overlap=200)
        chunks = text_splitter.split_text(data)
        embeddings = []

        for chunk in chunks:
            # try:
            embed = get_embedding(chunk, api_key=self.voyage_api_key)
            embeddings.append((chunk, embed))
            # except Exception as e:
            #     print(f"Error generating embedding: {str(e)}")
            #     continue
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
                    {"image": image_data.decode(), "media_type": f"image/{_get_media_type(path)}"})

        response = requests.post(f"{self.url}/completion", json=payload)
        if response.status_code != 200:
            raise Exception(f"API call failed with status code: {response.status_code}")

        result = response.json()
        return result


