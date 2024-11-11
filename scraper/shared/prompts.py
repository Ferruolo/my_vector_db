from typing import List

# Easier than using a text file, probs not the smartest way to do this tho


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
You are a menu image scraper, tasked with taking in images of menus (and only menus), 
and processing the relevant text in the menu. You will take the following steps to extract data.

1. Print all text data you see on the menu, ignore all other images, etc. Simply just repeat the text.

2. Now that you have the text, you will treat the text as an input. You will simply
reformat the text in the JSON format given below
{'items': [
        {'name': 'item name', 'price': '1234', 'type': 'APP/ENTREE/DRINK'},
    ]
}
"""

PROMPT_extract_all_important_links = """<s>[INST] <<SYS>>
You are a specialized link analyzer focused on extracting relevant restaurant information links. Your responses must be precise and contain only the requested JSON format.

TASK DEFINITION:
Analyze provided URLs and extract only links containing essential restaurant information.

REQUIRED DATA CATEGORIES:
1. MENU INFORMATION
   - Food descriptions
   - Meal names
   - Pricing

2. RESTAURANT ATMOSPHERE
   - Ambiance descriptions
   - Style and setting
   - Overall experience

3. LOCATION CONTEXT
   - Area description
   - Neighborhood characteristics
   - Local environment

4. LANDING PAGES
   - Main restaurant homepage
   - Primary information hub
   - Core service pages

EXCLUDE:
- Invalid or non-functioning links
- Administrative pages (terms of service, careers, etc)
- Any links not related to the categories above

JSON OUTPUT FORMAT:

"{
    "links": ["https://url/endpoint1", "https://url/endpoint2", "https://url/endpoint3"]
}"

If no relevant links found, return: {"links": []}
<</SYS>>

ANALYZE THESE LINKS:
"""


def format_extract_all_important_links(data: List[str]) -> str:
    prompt = PROMPT_extract_all_important_links + '\n'.join(data)
    prompt += """ [/INST]</s>"""
    return prompt

def format_extract_menu_data(data: List[str]) -> str:
    prompt = PROMPT_extract_menu_data + '\n'.join(data)
    return prompt