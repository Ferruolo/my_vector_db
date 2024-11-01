import requests
from bs4 import BeautifulSoup
import re

name = "LEXINGTON PUBLIK "

search_query = f'"{name}" restaurant website'
search_url = f"https://www.google.com/search?q={search_query}"

headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
}

content = requests.get(search_url, headers=headers)
soup = BeautifulSoup(content.content, 'html.parser')


with open("test.html", 'w') as f:
    f.write(soup.prettify())

rhs = soup.find("div", attrs={"id": "rhs"})

rhs.prettify()
company_name = rhs.find("div", attrs={"data-attrid": "title"}).text
links = rhs.find_all('a')
website_link = list(filter(lambda x : x.text == 'Website', links))[0].get('href')


subtitle = rhs.find("div", attrs={"data-attrid": "subtitle"})
price_string = list(subtitle.children)[4].text

pattern = r'\$?(\d+)\s*[-–—]\s*(\d+)'

match = re.search(pattern, price_string)
if match:
    low_price = int(match.group(1))  # 20
    high_price = int(match.group(2))  # 30


