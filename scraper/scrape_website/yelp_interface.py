import requests
from dotenv import load_dotenv
import os

load_dotenv()

class YelpInterface:
    def __init__(self):
        self.api_key = os.getenv("YELP_API_KEY")
        self.search_url = "https://api.yelp.com/v3/businesses/search"
        self.headers = {"accept": "application/json", "Authorization": f"Bearer {self.api_key}"}


    def get_website_from_coords(self, name, lat, lon, city="New York", address="", limit=5):
        response = requests.get(f"{self.search_url}?latitude={lat}&longitude={lon}&limit={limit}&term={name}", headers=self.headers)
        return response.json()