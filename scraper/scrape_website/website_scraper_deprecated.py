import io
from io import BytesIO
from typing import List
from urllib.parse import urljoin

import PyPDF2
import pytesseract
import requests
from PIL import Image
from bs4 import BeautifulSoup

from scrape_website.webscraper import Playwright
from shared.bloomfilter import BloomFilter
from shared.helpers import extract_base_url, drop_repeated_newline_regex, is_internal_link, is_toast_tab_link
from shared.unique_search_container import UniqueSearchContainer


def normalize_links(links: List[str], base_url: str) -> List[str]:
    base_url = base_url.rstrip('/')
    normalized_links = set()

    for link in links:
        if not link:
            continue

        if any([
            link.startswith('#'),
            link.startswith('mailto:'),
            link.startswith('tel:'),
            link == '/'
        ]):
            continue

        link = link.lstrip('/')

        if link.startswith('http://') or link.startswith('https://'):
            normalized_links.add(link)
            continue

        if link:
            normalized_links.add(f"{base_url}/{link}")

    return list(normalized_links)


def get_image_text(image_url: str) -> str:
    try:
        image_content = requests.get(image_url).content
        image = Image.open(BytesIO(image_content))
        if image.mode != 'RGB':
            image = image.convert('RGB')

        extracted_text = pytesseract.image_to_string(image)
        ascii_text = ''.join(char for char in extracted_text if ord(char) < 128)
        return ascii_text.strip()
    except Exception as e:
        print(f"Error {e} getting image {image_url}")


def extract_pdf_text(pdf_url: str) -> str:
    try:
        response = requests.get(pdf_url)
        if response.status_code != 200:
            return f"Error downloading PDF: {response.status_code}"
        pdf_file = io.BytesIO(response.content)
        pdf_reader = PyPDF2.PdfReader(pdf_file)
        text = ""
        for page in pdf_reader.pages:
            text += page.extract_text() + "\n"
        return text
    except Exception as e:
        return f"Error processing PDF: {str(e)}"


def is_pdf_link(url: str) -> bool:
    try:
        response = requests.head(url)
        content_type = response.headers.get('content-type', '').lower()
        if 'application/pdf' in content_type:
            return True
    except:
        pass
    return url.lower().endswith('.pdf')


async def scrape_all_text(url: str, scraper: Playwright):
    try:
        await scraper.goto(url)
        soup = await scraper.get_page_soup()
        main_element = soup.find('body')
        full_text = ""
        for div in main_element.find_all('div'):
            full_text += div.text + '\n'

        images = soup.find_all('imgs')
        for image in images:
            full_text += get_image_text(image)
        return full_text

    except Exception as e:
        print(f"Error making request: {e}")
        return ""


def get_all_links(website_link: str) -> List[str]:
    base_url = extract_base_url(website_link)
    search_container = UniqueSearchContainer(1000, 40, useDFS=False)
    search_container.push(website_link)
    urls = [website_link]
    url_filter = BloomFilter(1000, 100)
    url_filter.add_item(website_link)
    while not search_container.is_empty():
        link = search_container.pop()
        try:
            content = requests.get(link).content
            soup = BeautifulSoup(content, "html.parser")
            links = [link.get('href') for link in soup.find_all('a') if link.get('href')]
            links = list(filter(lambda x: is_internal_link(x, base_url), links))
            links = normalize_links(links, base_url)

            for link in links:
                search_container.push(link)
                if not url_filter.get_item(link):
                    urls.append(link)
                    url_filter.add_item(link)
        except Exception as e:
            print(f"\nError processing {link}: {str(e)}")
    return urls


def get_full_data(website_link: str) -> str:
    full_context = ""
    base_url = extract_base_url(website_link)
    search_container = UniqueSearchContainer(1000, 40, useDFS=False)
    search_container.push(website_link)

    while not search_container.is_empty():
        link = search_container.pop()
        print(f"Scraping {link}")
        if is_toast_tab_link(link):
            full_context += link
        if is_pdf_link(link):
            pdf_text = extract_pdf_text(link)
            full_context += f"{pdf_text}"
            continue
        try:
            content = requests.get(link).content
            soup = BeautifulSoup(content, "html.parser")
            text = str(soup.getText())
            title = str(soup.find('title').text) if soup.find('title') else "No Title"
            links = [link.get('href') for link in soup.find_all('a') if link.get('href')]
            links = list(filter(lambda x: is_internal_link(x, base_url), links))
            links = normalize_links(links, base_url)

            for link in links:
                search_container.push(link)

            images = [link.get('src') for link in soup.find_all('img') if link.get('src')]
            images = [urljoin(base_url, img) for img in images]

            full_context += f"\n\n{title}\n{text}"
            for image in images:
                full_context += get_image_text(image)

            full_context = drop_repeated_newline_regex(full_context)
            # sentence_set = set()
            # sentences = []
            # for line in full_context.split("\n"):
            #     for sent in line.split("."):
            #         hash = md5(sent.encode()).hexdigest()
            #         if hash not in sentence_set:
            #             sentence_set.add(hash)
            #             sentences.append(sent)
            #

        except Exception as e:
            full_context += f"\nError processing {link}: {str(e)}"

    return full_context
