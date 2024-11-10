import requests
from dotenv import load_dotenv
import os
from bs4 import BeautifulSoup
import re

load_dotenv()


def extract_url(input_string):
    pattern = r'url=([^&]+)'
    match = re.search(pattern, input_string)

    if match:
        import urllib.parse
        return urllib.parse.unquote(match.group(1))

    return None


class YelpInterface:
    def __init__(self):
        self.api_key = os.getenv("YELP_API_KEY")
        self.search_url = "https://api.yelp.com/v3/businesses/search"
        self.headers = {"accept": "application/json", "Authorization": f"Bearer {self.api_key}"}
        self.website_headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) '
                          'Chrome/120.0.0.0 Safari/537.36',
            'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
            'Accept-Language': 'en-US,en;q=0.5', 'Connection': 'keep-alive', 'Upgrade-Insecure-Requests': '1',
            'Cache-Control': 'max-age=0', }

    def get_website_from_coords(self, name, lat, lon, city="New York", address="", limit=1):
        response = requests.get(f"{self.search_url}?latitude={lat}&longitude={lon}&limit={limit}&term={name}",
                                headers=self.headers)
        return response.json()

    def extract_url(self, yelp_response: dict):
        yelp_url = yelp_response['url']
        response = requests.get(yelp_url)
        soup = BeautifulSoup(response.content, "html.parser")
        with open("sample.html", 'w') as f:
            f.write(soup.prettify())
        islands = soup.find_all('div', attrs={'data-testid': 'cookbook-island'})

        selected = list(filter(lambda island: "get directions" in island.text.lower(), islands))[0]
        dirty_url = selected.find_all('a')[0].get('href')
        url = extract_url(dirty_url)
        return url
