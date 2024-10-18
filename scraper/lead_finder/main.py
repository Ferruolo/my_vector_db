import requests
import dotenv
dotenv.load_dotenv()

def search_nearby_places(api_key, lat, long, radius):
    url = "https://places.googleapis.com/v1/places:searchNearby"

    headers = {
        "Content-Type": "application/json",
        "X-Goog-Api-Key": api_key,
        "X-Goog-FieldMask": "places.displayName"
    }

    payload = {
        "includedTypes": ["restaurant"],
        "maxResultCount": 10,
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
api_key =
latitude = 37.7937
longitude = -122.3965
radius = 500.0

result = search_nearby_places(api_key, latitude, longitude, radius)
print(result)
