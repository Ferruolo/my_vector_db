from cassandra.cluster import Cluster
from cassandra.auth import PlainTextAuthProvider
from concurrent.futures import ThreadPoolExecutor
import time


def get_cassandra_session(host='localhost',
                          port=9042,
                          keyspace='restaurant_inspections',
                          username=None,
                          password=None):
    auth_provider = None
    if username and password:
        auth_provider = PlainTextAuthProvider(username=username, password=password)

    cluster = Cluster(
        contact_points=[host],
        port=port,
        auth_provider=auth_provider,
        protocol_version=4
    )
    return cluster.connect(keyspace)


def batch_delete(session, batch_size=1000, max_workers=10):
    """
    Parallel batch deletion using thread pool
    """
    # Get all item_ids where dba is empty or ''
    query = "SELECT item_id FROM restaurant_inspections WHERE dba = '' ALLOW FILTERING"
    print("Fetching rows to delete...")
    rows = session.execute(query)

    # Prepare delete statement
    delete_stmt = session.prepare("DELETE FROM restaurant_inspections WHERE item_id = ?")

    # Process in batches using thread pool
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        batch = []
        count = 0
        for row in rows:
            batch.append(row.item_id)
            if len(batch) >= batch_size:
                # Execute batch in parallel
                futures = [
                    executor.submit(session.execute, delete_stmt, [item_id])
                    for item_id in batch
                ]
                # Wait for batch to complete
                for future in futures:
                    future.result()
                count += len(batch)
                print(f"Deleted batch of {len(batch)} rows. Total deleted: {count}")
                batch = []
                time.sleep(1)  # Prevent overwhelming the cluster

        # Process remaining rows
        if batch:
            futures = [
                executor.submit(session.execute, delete_stmt, [item_id])
                for item_id in batch
            ]
            for future in futures:
                future.result()
            count += len(batch)
            print(f"Deleted final batch of {len(batch)} rows. Total deleted: {count}")


if __name__ == "__main__":
    try:
        session = get_cassandra_session(
            host='localhost',  # change if your host is different
            keyspace='restaurant_inspections'
        )

        batch_delete(session)
        session.cluster.shutdown()

    except Exception as e:
        print(f"Error: {str(e)}")