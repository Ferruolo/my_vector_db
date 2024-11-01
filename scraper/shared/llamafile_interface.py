import requests
from typing import Dict, List, Optional, Union
import json

PROMPT_extract_important_image_urls = """
"""

PROMPT_extract_image_data = """

"""

PROMPT_test = """
"""




class LlamaFileWrapper:
    def __init__(self, host: str = "localhost", port: int = 8080):
        self.base_url = f"http://{host}:{port}/v1"

    def _make_request(self, endpoint: str, payload: Dict) -> Dict:
        url = f"{self.base_url}/{endpoint}"
        try:
            response = requests.post(url, json=payload)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            raise Exception(f"Error making request to {endpoint}: {str(e)}")

    def get_completion(
            self,
            prompt: str,
            system_prompt: Optional[str] = None,
            temperature: float = 0.7,
            max_tokens: int = 1000,
            stop: Optional[Union[str, List[str]]] = None,
    ) -> str:
        messages = []
        if system_prompt:
            messages.append({"role": "system", "content": system_prompt})
        messages.append({"role": "user", "content": prompt})

        payload = {
            "messages": messages,
            "temperature": temperature,
            "max_tokens": max_tokens,
        }

        if stop:
            payload["stop"] = stop

        response = self._make_request("chat/completions", payload)
        return response["choices"][0]["message"]["content"]

    def get_embedding(self, text: Union[str, List[str]]) -> Union[List[float], List[List[float]]]:
        if isinstance(text, str):
            text = [text]

        payload = {
            "input": text,
            "model": "default"
        }

        response = self._make_request("embeddings", payload)
        embeddings = [data["embedding"] for data in response["data"]]

        return embeddings[0] if len(embeddings) == 1 else embeddings

    def health_check(self) -> bool:
        try:
            response = requests.get(f"{self.base_url}/models")
            return response.status_code == 200
        except:
            return False