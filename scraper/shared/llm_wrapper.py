import base64
from typing import Optional, List, Union
from anthropic import Anthropic
import os
import re
from dotenv import load_dotenv
import requests
from shared.prompts import PROMPT_extract_menu_data

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

    def make_call(self, prompt: str, system_prompt: Optional[str] = None,
                  image_paths: Optional[List[str]] = None) -> str:
        content = []
        if image_paths:
            for path in image_paths:
                image_data = self._encode_image(path)
                content.append({"type": "image",
                                "source": {"type": "base64", "media_type": f"image/{self._get_media_type(path)}",
                                           "data": image_data.decode()}})

        content.append({"type": "text", "text": prompt})

        message = self.client.messages.create(model=self.model_name, max_tokens=1024, messages=[
            {"role": "user", "content": content if image_paths else prompt}],
                                              system=system_prompt if system_prompt else "")
        return message.content[0].text


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

    def completion(self, prompt: str, system_prompt: str = "") -> str:
        payload = {
            "prompt": prompt,
            "system": system_prompt,
            "stream": False
        }

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


