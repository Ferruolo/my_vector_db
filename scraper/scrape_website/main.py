import requests
from shared.redis_intrerface import create_document_db_interface, create_redis_queue, create_redis_client
from bs4 import BeautifulSoup
import json
from shared.models import LocationData
from urllib.parse import urlparse
import PyPDF2
import re
import io

def remove_trailing_slash(url):
    return re.sub(r'/$', '', url)

def extract_text_from_pdf(pdf_data):
    pdf_file = io.BytesIO(pdf_data)
    reader = PyPDF2.PdfReader(pdf_file)
    text = ""
    for page in reader.pages:
        text += page.extract_text()
    return text


def extract_base_url(url):
    parsed_url = urlparse(url)
    base_url = f"{parsed_url.scheme}://{parsed_url.netloc}"
    return base_url


client = create_redis_client()

push, pop, length, is_empty = create_redis_queue(client, "search_queue")

item = pop()

data = LocationData(**json.loads(item))

soup = BeautifulSoup(requests.get(data.websiteUri).content, "html.parser")

clean_link = extract_base_url(data.websiteUri)

links = soup.find_all("a")
links = [link.get('href') for link in links]

local_links = [link for link in links if link and extract_base_url(link) == clean_link]
local_links = list(filter(lambda x: x != clean_link, local_links))

menu = local_links[1]
menu_soup = BeautifulSoup(requests.get(menu).content, "html.parser")
menu_soup_links = [x.get('href') for x in menu_soup.find_all('a')]
menu_soup_links = [link for link in menu_soup_links if link and extract_base_url(link) == clean_link]

example_url = "https://www.segoviameson.com/pdfs/menu_lunch.pdf"

menu = requests.get(example_url).content

