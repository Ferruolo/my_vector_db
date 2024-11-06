import pandas as pd
import spacy
from cassandra.cluster import Cluster
from sqlalchemy import false
from sqlalchemy.testing import fails

from scrape_website.google_scraper import get_comapny_data
from scrape_website.website_scraper import get_full_data, drop_repeated_newline_regex
from shared.llm_wrapper import LlamafileWrapper
from shared.redis_interface import create_redis_client, create_channel_interface
from llama_index.core.node_parser import SentenceSplitter
from shared.prompts import PROMPT_extract_menu_data

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
    llama = LlamafileWrapper()
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
    text_splitter = SentenceSplitter(chunk_size=512, chunk_overlap=50)

    if not fetch_item(index_name):
        put_item(index_name, 0)

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
        print("Fetched Rows")
        for name in df['name']:
            print(f"Fetching Data for Company {name}")
            try:
                company_name, website_link = get_comapny_data(name)

                string_data = get_full_data(website_link)
                # print(f"Len Company Data for {name}: {string_data}")
                string_data = drop_repeated_newline_regex(string_data)
                with open("data.txt", 'r') as f:
                    data = f.read() + string_data
                with open("data.txt", 'w') as f:
                    f.write(data)

                # print(string_data)
                # menu = llama.completion(string_data, system_prompt=PROMPT_extract_menu_data)
                # print("\n\n\n\n MENU \n ------------------------")
                # print(menu)
                # assert(false)
                # chunks = text_splitter.split_text(string_data)
            except Exception as e:
                print(f"{name} failed with {e}")



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
