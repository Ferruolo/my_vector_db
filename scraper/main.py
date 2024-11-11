import json
import uuid
from typing import List, Tuple
from uuid import uuid4

import pandas as pd
from cassandra.cluster import Cluster, Session

from scrape_website.website_scraper import get_all_links, is_pdf_link
from scrape_website.yelp_interface import YelpInterface
from shared.extra_apis import get_coordinates
from shared.helpers import drop_repeated_newline_regex
from shared.llm_wrapper import LlamafileWrapper, ClaudeWrapper
from shared.models import Menu, Location
from shared.prompts import format_extract_all_important_links
from shared.put_data_to_cassandra import insert_business, insert_menu_item, insert_business_location, insert_text_data
from shared.redis_interface import create_redis_client, create_channel_interface

table_name = "restaurant_inspections_indexed"
index_name = "SCRAPER_IDX"
incremental_constant = 100


def calc_price_magnitude(dollar_signs: str) -> int:
    return dollar_signs.count('$')


def put_menu_items(session, menu: Menu, biz_id):
    for item in menu.items:
        insert_menu_item(session, biz_id, item.name, item.type, item.price, item.description)


def put_biz_locations(session, locations: List[Location], biz_id):
    for location in locations:
        lat, long = get_coordinates(location)
        insert_business_location(session, uuid4(), biz_id, lat, long, location.building_number, location.street,
                                 location.room_number, location.city, location.state)


def put_chunks(session: Session, biz_id, embeddings: List[Tuple[str, List[float]]]):
    for text, embed in embeddings:
        insert_text_data(session, biz_id, text, embed)


def main() -> None:
    llama = LlamafileWrapper()
    claude = ClaudeWrapper()
    redis = create_redis_client()
    (put_item, delete_item, fetch_item) = create_channel_interface(redis, channel=0)
    cluster = Cluster()
    session = cluster.connect()
    table_name = "restaurant_inspections"
    session.set_keyspace('restaurant_inspections')

    prepped_db_call = session.prepare("""
        SELECT dba as name, cuisine_description, latitude, longitude, street, building FROM {} 
        WHERE row_index >= ? AND row_index < ?
        ALLOW FILTERING
    """.format(table_name))

    prepped_count_call = session.prepare("""SELECT count(*) as COUNT FROM {}""".format(table_name))

    if not fetch_item(index_name):
        put_item(index_name, 0)

    yelp = YelpInterface()

    while True:
        index_start = int(str(fetch_item(index_name).decode('utf-8')))
        current_index = int(session.execute(prepped_count_call).one().count)
        end_index = index_start + incremental_constant
        if index_start >= current_index:
            continue
        else:
            put_item(index_name, end_index)

        print(f"Fetching data for {index_start}-{end_index}")

        rows = session.execute(prepped_db_call, (index_start, end_index))
        data = [dict(row._asdict()) for row in rows]

        # Create DataFrame
        df = pd.DataFrame(data)
        for idx, row in df.iterrows():
            print(f"Fetching Data for Company {row['name']}")
            # try:
            biz_data = yelp.get_website_from_coords(row['name'], row['latitude'], row['longitude'],
                                                    address=f"{row['building']} {row['street']}")
            selected = biz_data['businesses'][0]
            with open("yelp_api.json", 'w') as f:
                f.write(json.dumps(biz_data))
            # Get all links
            url = yelp.extract_url(selected)
            links = get_all_links(url)
            print(links)
            print("Parsing with llama")
            result = llama.completion(format_extract_all_important_links(links))

            links = json.loads(result['content'])['links']

            all_text = ""
            for link in links:
                if is_pdf_link(link):
                    all_text += claude.extract_pdf_data(link)
                else:
                    all_text += link + '\n\n'

            all_text = drop_repeated_newline_regex(all_text)

            structured_data = claude.extract_structured_data(all_text)

            text_data = claude.get_embeddings(all_text)

            biz_id = uuid.uuid4()
            try:
                # Use try catch to add atomicity (makes it easier to keep everything straight)
                insert_business(session, biz_id, selected['name'], selected['id'], "pickup" in selected['transactions'],
                                "delivery" in selected['transactions'],
                                selected['rating'], calc_price_magnitude(selected['price']), selected['phone'], url)
                put_menu_items(session, structured_data.menu, biz_id)
                put_biz_locations(session, structured_data.locations, biz_id)
                put_chunks(session, biz_id, text_data)

            except KeyboardInterrupt:
                exit(1)
            except Exception as e:
                print(f"{row['name']} failed with {e}")  # TODO: Create list of all failures in redis


if __name__ == "__main__":
    main()
