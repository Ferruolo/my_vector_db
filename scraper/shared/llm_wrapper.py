import base64
from typing import Optional, List
from anthropic import Anthropic
import os
import re
from dotenv import load_dotenv

load_dotenv()

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

    def make_call(self, prompt: str, system_prompt: Optional[str] = None) -> str:
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
                                "source": {"type": "base64", "media_type": f"image/{self._get_media_type(path)}", "data": image_data.decode()}})

        content.append({"type": "text", "text": prompt})

        message = self.client.messages.create(model=self.model_name, max_tokens=1024,
                                              messages=[
                                                  {"role": "user", "content": content if image_paths else prompt}],
                                              system=system_prompt if system_prompt else "")
        return message.content[0].text


image_path = "./data/sample-menu.png"

claude = ClaudeWrapper(url="", api_key=os.environ.get("CLAUDE_API_KEY"))
result = claude.make_call(prompt=PROMPT_extract_menu_data, image_paths=[image_path])

