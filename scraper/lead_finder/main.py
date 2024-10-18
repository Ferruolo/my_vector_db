import requests
import dotenv
import os
import uuid
dotenv.load_dotenv()


start_lat = 40.8268
end_lat = 40.6973
start_long = -74.0588
end_long = -73.9181


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



def search_grid_area(lat_start, lat_end, long_start, long_end, radius, num_squares):






# Example usage
api_key = os.environ.get('GOOGLE_API_KEY')

radius = 500

result = search_nearby_places(api_key, latitude, longitude, radius)
# print(len(result))
if __name__ == "__main__":
