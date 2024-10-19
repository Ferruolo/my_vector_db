import requests
from shared.redis_intrerface import create_document_db_interface, create_redis_queue, create_redis_client
from bs4 import BeautifulSoup
import json
from shared.models import LocationData
from urllib.parse import urlparse


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


