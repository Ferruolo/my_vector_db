import requests
import dotenv
import os

dotenv.load_dotenv()


def search_nearby_places(api_key, lat, long, radius):
    url = "https://places.googleapis.com/v1/places:searchNearby"

    headers = {
        "Content-Type": "application/json",
        "X-Goog-Api-Key": api_key,
        "X-Goog-FieldMask": "places.displayName,places.formattedAddress,places.types,places.websiteUri"
    }

    payload = {
        "includedTypes": ["restaurant"],
        "maxResultCount": 20,
        "locationRestriction": {
            "circle": {
                "center": {
                    "latitude": lat,
                    "longitude": long
                },
                "radius": radius
            }
        }
    }

    response = requests.post(url, json=payload, headers=headers)

    if response.status_code == 200:
        return response.json()
    else:
        return f"Error: {response.status_code}, {response.text}"


# Example usage
api_key = os.environ.get('GOOGLE_API_KEY')
latitude = 40.7484
longitude = -73.9857
radius = 500

result = search_nearby_places(api_key, latitude, longitude, radius)
print(len(result))
