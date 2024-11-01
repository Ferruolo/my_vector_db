from cassandra.cluster import Cluster
from cassandra.query import SimpleStatement
from cassandra.policies import RetryPolicy
from time import sleep
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def migrate_data(source_table, target_table, keyspace, batch_size=1000, max_retries=3):
    """
    Migrate data between Cassandra tables with error handling and retries
    """
    try:
        # Connect to cluster
        cluster = Cluster(['localhost'])  # Modify host as needed
        session = cluster.connect(keyspace)

        # Get total count (approximate)
        count_query = f"SELECT COUNT(*) FROM {source_table}"
        total_rows = session.execute(count_query).one().count

        # Prepare statements
        select_query = f"SELECT * FROM {source_table}"
        insert_query = f"INSERT INTO {target_table} JSON ?"

        # Configure paging
        statement = SimpleStatement(select_query, fetch_size=batch_size)

        current_retry = 0
        processed_rows = 0
        failed_rows = []

        while True:
            try:
                rows = session.execute(statement)

                for row in rows:
                    try:
                        # Convert row to JSON format
                        json_data = '{'
                        for col_name in row._fields:
                            value = getattr(row, col_name)
                            if isinstance(value, str):
                                json_data += f'"{col_name}":"{value}",'
                            else:
                                json_data += f'"{col_name}":{value},'
                        json_data = json_data.rstrip(',') + '}'

                        # Insert into new table
                        session.execute(insert_query, [json_data])
                        processed_rows += 1

                        if processed_rows % 1000 == 0:
                            logger.info(f"Processed {processed_rows}/{total_rows} rows")

                    except Exception as e:
                        logger.error(f"Error processing row: {e}")
                        failed_rows.append(row)
                        continue

                if not rows.has_more_pages:
                    break

            except Exception as e:
                current_retry += 1
                if current_retry >= max_retries:
                    logger.error(f"Max retries reached. Error: {e}")
                    break

                logger.warning(f"Retry {current_retry}/{max_retries}. Error: {e}")
                sleep(5)  # Wait before retry

        # Handle failed rows
        if failed_rows:
            logger.warning(f"Failed to process {len(failed_rows)} rows")
            with open('failed_rows.log', 'w') as f:
                for row in failed_rows:
                    f.write(str(row) + '\n')

        logger.info(f"Migration completed. Processed {processed_rows}/{total_rows} rows")

    finally:
        cluster.shutdown()


if __name__ == "__main__":
    # Configuration
    SOURCE_TABLE = "restaurant_inspections_indexed"
    TARGET_TABLE = "restaurant_inspections"
    KEYSPACE = "restaurant_inspections"  # Replace with your keyspace

    migrate_data(SOURCE_TABLE, TARGET_TABLE, KEYSPACE)