import requests
from shared.redis_intrerface import create_redis_queue, create_redis_client, create_document_db_interface
from bs4 import BeautifulSoup
import json
from shared.models import LocationData
from urllib.parse import urlparse
import PyPDF2
import re
import io
from queue import Queue
from shared.unique_search_container import UniqueSearchContainer
from redis import Redis


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
    base_url = remove_trailing_slash(base_url)
    return base_url


def is_valid_url(url):
    if url is None:
        return False
    pattern = r'^(https?:\/\/)?([\da-z\.-]+)\.([a-z\.]{2,6})([\/\w \.-]*)*\/?$'
    return bool(re.match(pattern, url))

def ends_with_pdf(url):
    return url.lower().endswith('.pdf')

# assumes base-url is already cleaned for efficiency's sake
def get_all_links(page: BeautifulSoup, base_url: str):
    links = [a.get('href') for a in page.find_all('a')]
    links = list(filter(lambda x: is_valid_url(x), links))
    links = list(filter(lambda x: extract_base_url(x) != base_url, links))
    return links


def get_text(page: BeautifulSoup):
    text = list()
    search = Queue()
    search.put(page)

    while not search.empty():
        soup: BeautifulSoup = search.get()
        if soup.string:
            text.append(soup.string.strip())
        for child in soup.children:
            if isinstance(child, BeautifulSoup):
                search.put(child)
    return '\n'.join(text)


def search_data(data: LocationData):
    base_url = extract_base_url(data.websiteUri)
    search_container = UniqueSearchContainer(200, 50, useDFS=False)
    full_text = ""
    while not search_container.is_empty():
        current_url = search_container.pop()
        text = ""
        link_content = requests.get(current_url).content
        if ends_with_pdf(current_url):
            text = extract_text_from_pdf(link_content)
        else:
            soup = BeautifulSoup(link_content, "html.parser")
            text = get_text(soup)
            links = get_all_links(soup, base_url)
            for link in links:
                search_container.push(link)
        full_text += f"Current URL: {current_url}\n\n {text} \n\n"
    return full_text


def main_program(redis_client: Redis):
    push, pop, length, is_empty = create_redis_queue(redis_client, "search_queue")
    put_item, delete_item, fetch_item = create_document_db_interface(redis_client)
    while not is_empty():
        try:
            item = pop()
            data = LocationData(**json.loads(item))
            key = extract_base_url(data.websiteUri)
            item_text = search_data(data)
            put_item.set(key, item_text)
        except KeyboardInterrupt:
            print("Keyboard Interrupt detected, breaking")
        except Exception as e:
            print("Exception: ", e)


# We want to keep each process on a single thread so that it
# can be optimized easily from the infrastructure side
if __name__ == '__main__':
    client = create_redis_client()
    main_program(client)
