import requests
from typing import Dict, List, Optional, Union

PROMPT_extract_important_image_urls = """
You are a web scraper tasked with analyzing the html output of a webpage. Given the 
html of a webpage, you are tasked with identifying all important image links. You 
will then return these image links if and only if they were in the original html, in the following JSON. 
Please note that some important image links may link to a CDN or something similar.

Make sure that you return your response in the following format, without any excess text
{
    'links': [https://myurl.com/path/to/image, https://myurl.com/path/to/image2] 
}
"""

PROMPT_extract_menu_data = """
You are a menu scraper tasked with analyzing images and PDFs of menus. 
You will take in the menu as an input, and you will return all menu items in the following JSON format:

{'items': [
        {'name': 'item name', 'price': '1234', 'type': 'APP/ENTREE/DRINK'},
    ]
}
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

