from redis import Redis
import requests
import dotenv
import os
from shared.redis_intrerface import create_redis_client, create_redis_queue
import json

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
        try:
            return response.json()['places']
        except Exception as e:
            print(response.json())
            print("Found None")
            return None
    else:
        print(f"Error: {response.status_code}, {response.text}")
        return None


def search_grid_area(api_key: str, client: Redis, lat_start, lat_end, long_start, long_end, radius, num_squares):
    push, pop, length, is_empty = create_redis_queue(client, "search_queue")
    diff_x = (long_end - long_start) / num_squares
    diff_y = (lat_end - lat_start) / num_squares
    lat = lat_start
    long = long_start
    #TODO: Not perfect, but works for now
    idx = 0
    while lat > lat_end:
        while long < long_end:
            print(f"{idx}: Fetching data for ({lat:.04f}, {long:.04f})")
            data = search_nearby_places(api_key, lat, long, radius)
            if data is not None:
                [push(json.dumps(x)) for x in data]
            idx += 1
            long += diff_x
            # time.sleep(1)
        lat -= diff_y
        long = long_start


if __name__ == '__main__':
    api_key = os.environ.get('GOOGLE_API_KEY')
    client = create_redis_client()
    print("HELLO WORLD")
    search_grid_area(api_key, client, start_lat, end_lat, start_long, end_long, 500, 1000)
