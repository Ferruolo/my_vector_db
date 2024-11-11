import requests
from bs4 import BeautifulSoup
import re

headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
}

price_match_pattern = r'\$?(\d+)\s*[-–—]\s*(\d+)'


def get_comapny_data(name: str):
    search_query = f'{name} restaurant website'.replace(' ', '+')
    search_url = f"https://www.google.com/search?q={search_query}"
    print(search_url)
    content = requests.get(search_url, headers=headers)
    soup = BeautifulSoup(content.content, 'html.parser')

    rhs = soup.find("div", attrs={"id": "rhs"})
    if not rhs is None:

        company_name = name
        links = rhs.find_all('a')
        website_link = list(filter(lambda x: x.text == 'Website', links))[0].get('href')

        return company_name, website_link
    else:
        links = [x.get('href') for x in soup.find('div', attrs={'id': 'rso'}).find_all('a')]
        website_link = links[0]
        return name, website_link
