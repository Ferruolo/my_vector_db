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


PROMPT_extract_all_important_links = """
I am building a database to help people identify the best place to eat based on certain requirements.
In order to do this, I will have to visit a series of links and scrape data from them. However, 
as each page contains such a large set of links, I will need to select a small subset that contain valuable data.
Specifically, I need to extract data that is relavent to the following categories:

1. What is on the menus (food descriptions, meal names, cost, etc)
2. What is the "vibe" of the restaurant (is is quaint, fun, romantic, etc)
3. What is the area the restaurant is in. Is it busy, quiet, dangerous, etc?

I want to avoid the following links:
1. Any "hallucinated, fake, or dead links"
2. Terms of service pages, careers pages, or any extraneous information
3. Any links not specifically mentioned above that do not specifically coordinate the the requested columns.

You must return the data in the following JSON format:
{
    "links": ["https://url/endpoint1", "https://url/endpoint1", "https://url/endpoint1"]
}
"""


