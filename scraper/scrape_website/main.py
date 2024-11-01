from cassandra.cluster import Cluster, BatchStatement
from cassandra.auth import PlainTextAuthProvider
import pandas as pd

cluster = Cluster()
session = cluster.connect()
session.set_keyspace('restaurant_inspections')

table_name = "restaurant_inspections_indexed"





prepared_stmt = session.prepare("""
    SELECT dba as name, cuisine_description, latitude, longitude, street FROM {} 
    WHERE row_index >= ? AND row_index < ?
    ALLOW FILTERING
""".format(table_name))

x = 0

    # Execute with parameters
rows = session.execute(prepared_stmt, (x, x + 100))

data = [dict(row._asdict()) for row in rows]

# Create DataFrame
df = pd.DataFrame(data)
df.head()

