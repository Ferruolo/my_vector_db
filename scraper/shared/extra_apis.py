import requests
import time
from typing import Tuple


from shared.models import Location


def get_coordinates(loc: Location) -> Tuple[float, float]:
    base_url = "https://nominatim.openstreetmap.org/search"
    address = f"{loc.building_number} {loc.street}, {loc.city}, {loc.state}"

    params = {
        'q': address,
        'format': 'json',
        'limit': 1,
        'user-agent': 'Certainty DB'
    }

    try:
        time.sleep(1)
        response = requests.get(base_url, params=params, headers={'User-Agent': "Certainty DB"})
        print(response.content)
        response.raise_for_status()
        results = response.json()

        if not results:
            raise ValueError(f"No coordinates found for address: {address}")

        latitude = float(results[0]['lat'])
        longitude = float(results[0]['lon'])

        return latitude, longitude

    except requests.RequestException as e:
        raise requests.RequestException(f"API request failed: {str(e)}")
