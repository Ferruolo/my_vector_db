import requests
from Cython.Build.BuildExecutable import LINKCC
from bs4 import BeautifulSoup
from shared.unique_search_container import UniqueSearchContainer
from typing import Optional, List
from urllib.parse import urlparse

website_link = "https://www.lexingtonpublicknyc.com/"

from urllib.parse import urlparse
from typing import Optional


def extract_base_url(url: str) -> Optional[str]:
    try:
        parsed = urlparse(url)
        if parsed.scheme and parsed.netloc:
            return f"{parsed.scheme}://{parsed.netloc}"
        return None
    except Exception:
        return None


def is_internal_link(url: str, base_site: str) -> bool:
    url_base = extract_base_url(url)
    if not url_base:
        return True
    site_base = extract_base_url(base_site)
    if not site_base:
        return False
    return url_base.lower() == site_base.lower()


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

full_context = ""
#
# search_container = UniqueSearchContainer(200, 40, useDFS=False)
# search_container.push(website_link)


link = website_link
content = requests.get(link).content
soup = BeautifulSoup(content, "html.parser")
text = str(soup.getText())
title = str(soup.find('title').text)

links = [link.get('href') for link in soup.find_all('a')]
base_url = extract_base_url(website_link)
links = list(filter(lambda x: is_internal_link(x, base_url), links))
links = normalize_links(links, base_url)
