from cassandra.cluster import Cluster, BatchStatement
from cassandra.auth import PlainTextAuthProvider
import pandas as pd

from scrape_website.google_scraper import get_comapny_data
from shared.redis_interface import create_redis_client, create_channel_interface
from shared.unique_search_container import UniqueSearchContainer


cluster = Cluster()
session = cluster.connect()
session.set_keyspace('restaurant_inspections')

table_name = "restaurant_inspections_indexed"
index_name = "SCRAPER_IDX"
incremental_constant = 100


def main() -> None:

    redis = create_redis_client()
    (put_item, delete_item, fetch_item) = create_channel_interface(redis, channel=0)
    cluster = Cluster()
    session = cluster.connect()
    table_name = "restaurant_inspections_indexed"
    session.set_keyspace('restaurant_inspections')

    preped_db_call = session.prepare("""
        SELECT dba as name, cuisine_description, latitude, longitude, street FROM {} 
        WHERE row_index >= ? AND row_index < ?
        ALLOW FILTERING
    """.format(table_name))

    preped_count_call = session.prepare("""SELECT count(*) as COUNT FROM {}""".format(table_name))

    if not fetch_item(index_name):
        put_item(index_name, 0)

    while True:
        index_start = int(str(fetch_item))
        current_index = session.execute(preped_count_call).one().COUNT
        end_index = current_index + incremental_constant
        if index_start >= current_index:
            continue
        else:
            put_item(index_name, end_index)

        # Execute with parameters
        rows = session.execute(preped_db_call, (str(index_start), str(end_index)))

        data = [dict(row._asdict()) for row in rows]

        # Create DataFrame
        df = pd.DataFrame(data)
        company_name, website_link, low_price, high_price = df['name'].apply(lambda x : get_comapny_data(x))

        string_data = get_comapny_data(company_name)
        final_data = {
            "company_name": company_name,
            "website_link": website_link,
            "low_price": low_price,
            "high_price": high_price,
            "string_data": string_data,
        }


if __name__ == "__main__":
    main()
