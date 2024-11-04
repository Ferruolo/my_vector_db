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
You are a menu scraper tasked with analyzing images and PDFs of menus. 
You will take in the menu as an input, and you will return all menu items in the following JSON format:

{'items': [
        {'name': 'item name', 'price': '1234', 'type': 'APP/ENTREE/DRINK'},
    ]
}
"""