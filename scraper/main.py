from cassandra.cluster import Cluster, BatchStatement
from cassandra.auth import PlainTextAuthProvider
import pandas as pd

from scrape_website.google_scraper import get_comapny_data
from shared.redis_interface import create_redis_client, create_channel_interface
from shared.unique_search_container import UniqueSearchContainer
from scrape_website.website_scraper import get_full_data
import spacy


table_name = "restaurant_inspections_indexed"
index_name = "SCRAPER_IDX"
incremental_constant = 100

def chunk_item(my_string: str, nlp: spacy, max_chunk_size: int = 1000, overlap: int = 100):
    doc = nlp(my_string)
    chunks = []
    current_chunk = []
    current_size = 0
    last_end = 0
    for sent in doc.sents:
        sentence_text = sent.text.strip()
        sentence_len = len(sentence_text)

        if current_size + sentence_len > max_chunk_size and current_chunk:
            # Join the current chunk and add it to chunks
            chunks.append(" ".join(current_chunk))

            # Start new chunk with overlap
            if chunks:
                # Find sentences that fit within overlap size
                overlap_text = chunks[-1][-overlap:]
                current_chunk = [overlap_text]
                current_size = len(overlap_text)
            else:
                current_chunk = []
                current_size = 0

        # Add sentence to current chunk
        current_chunk.append(sentence_text)
        current_size += sentence_len

    # Add the last chunk if it's not empty
    if current_chunk:
        chunks.append(" ".join(current_chunk))

    return chunks


def main() -> None:

    redis = create_redis_client()
    (put_item, delete_item, fetch_item) = create_channel_interface(redis, channel=0)
    cluster = Cluster()
    session = cluster.connect()
    table_name = "restaurant_inspections_indexed"
    session.set_keyspace('restaurant_inspections')

    prepped_db_call = session.prepare("""
        SELECT dba as name, cuisine_description, latitude, longitude, street FROM {} 
        WHERE row_index >= ? AND row_index < ?
        ALLOW FILTERING
    """.format(table_name))

    prepped_count_call = session.prepare("""SELECT count(*) as COUNT FROM {}""".format(table_name))

    if not fetch_item(index_name):
        put_item(index_name, 0)

    while True:
        print("Start Loop")
        index_start = int(str(fetch_item(index_name).decode('utf-8')))
        current_index = int(session.execute(prepped_count_call).one().count)
        end_index = index_start + incremental_constant
        if index_start >= current_index:
            continue
        else:
            put_item(index_name, end_index)
        print(f"Fetching data for {index_start}-{end_index}")
        # Execute with parameters
        rows = session.execute(prepped_db_call, (index_start, end_index))
        print("Fetched Rows")
        data = [dict(row._asdict()) for row in rows]

        # Create DataFrame
        df = pd.DataFrame(data)
        print("Fetched Rows")
        for name in df['name']:
            company_name, website_link, low_price, high_price = get_comapny_data(name)
            print(company_name, website_link, low_price, high_price)
            string_data = get_full_data(website_link)
            print(len(string_data))
        # chunks = chunk_item(string_data, nlp)
        #
        # final_data = {
        #     "company_name": company_name,
        #     "website_link": website_link,
        #     "low_price": low_price,
        #     "high_price": high_price,
        #     "string_data": string_data,
        # }


if __name__ == "__main__":
    main()
