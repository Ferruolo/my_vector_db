from cassandra.cluster import Cluster, BatchStatement
from cassandra.auth import PlainTextAuthProvider
from cassandra.cqltypes import LongType, IntegerType
from cassandra.query import dict_factory
from datetime import datetime
import pandas as pd
import numpy as np
import os


class RestaurantDataMigration:
    def __init__(self, contact_points=['localhost'], port=9042, username=None, password=None):
        """Initialize connection to Cassandra cluster."""
        if username and password:
            auth_provider = PlainTextAuthProvider(username=username, password=password)
            self.cluster = Cluster(contact_points, port=port, auth_provider=auth_provider)
        else:
            self.cluster = Cluster(contact_points, port=port)

        self.session = self.cluster.connect()

        self.session.execute("""
            CREATE KEYSPACE IF NOT EXISTS restaurant_inspections 
            WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1}
        """)

        self.session.set_keyspace('restaurant_inspections')
        self.create_table()
        self.insert_statement = self.session.prepare("""
            INSERT INTO restaurant_inspections (
                camis, inspection_date, dba, boro, building, street, zipcode,
                phone, cuisine_description, action, violation_code,
                violation_description, critical_flag, score, grade,
                grade_date, record_date, inspection_type, latitude,
                longitude, community_board, council_district, census_tract,
                bin, bbl
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """)



    def create_table(self):
        """Create the restaurant inspections table."""
        self.session.execute("""
            CREATE TABLE IF NOT EXISTS restaurant_inspections (
                camis bigint,
                inspection_date timestamp,
                dba text,
                boro text,
                building text,
                street text,
                zipcode decimal,
                phone bigint,
                cuisine_description text,
                action text,
                violation_code text,
                violation_description text,
                critical_flag text,
                score decimal,
                grade text,
                grade_date timestamp,
                record_date timestamp,
                inspection_type text,
                latitude decimal,
                longitude decimal,
                community_board decimal,
                council_district decimal,
                census_tract decimal,
                bin decimal,
                bbl decimal,
                PRIMARY KEY ((camis))
            )
        """)

        self.session.execute("CREATE INDEX IF NOT EXISTS idx_cuisine ON restaurant_inspections (cuisine_description)")

    def clean_data(self, df):
        """Clean and prepare DataFrame for insertion."""
        # Create a copy to avoid modifying original data
        # Convert date strings to datetime objects
        date_columns = ['INSPECTION DATE', 'GRADE DATE', 'RECORD DATE']
        for col in date_columns:
            if col in df.columns:
                df[col] = pd.to_datetime(df[col])

        # Convert numeric strings to appropriate types

        numeric_columns = {
            'CAMIS': ('integer', 0),  # IDs should be integers
            'ZIPCODE': ('integer', 0),  # ZIP codes are integers
            'PHONE': ('integer', 0),  # Phone numbers are integers
            'SCORE': ('integer', 0),  # Scores are integers
            'Latitude': ('float', 0.0),  # Latitude needs decimal precision
            'Longitude': ('float', 0.0),  # Longitude needs decimal precision
            'Community Board': ('integer', 0),  # Board numbers are integers
            'Council District': ('integer', 0),  # Districts are integers
            'Census Tract': ('integer', 0),  # Census tracts are integers
            'BIN': ('integer', 0),  # Building numbers are integers
            'BBL': ('integer', 0)  # Borough/Block/Lot are integers
        }

        for col, (type_name, default_value) in numeric_columns.items():
            if col in df.columns:
                # Convert to string first to handle inconsistent formats
                df[col] = df[col].astype(str)

                def safe_convert(val):
                    try:
                        if pd.isna(val) or val == '' or val.lower() == 'nan':
                            return default_value
                        # For integer types
                        if type_name == 'integer':
                            return int(float(val))
                        # For float types
                        elif type_name == 'float':
                            return float(val)
                        return default_value
                    except (ValueError, TypeError):
                        return default_value

                df[col] = df[col].apply(safe_convert)

        return df

    def prepare_row(self, row):
        """Convert a pandas row to a tuple of values for Cassandra insertion."""
        return (
            row['CAMIS'],
            row['INSPECTION DATE'],
            row['DBA'],
            row['BORO'],
            row['BUILDING'],
            row['STREET'],
            row['ZIPCODE'],
            row['PHONE'],
            row['CUISINE DESCRIPTION'],
            row['ACTION'],
            row['VIOLATION CODE'],
            row['VIOLATION DESCRIPTION'],
            row['CRITICAL FLAG'],
            row['SCORE'],
            row['GRADE'],
            row['GRADE DATE'],
            row['RECORD DATE'],
            row['INSPECTION TYPE'],
            row['Latitude'],
            row['Longitude'],
            row['Community Board'],
            row['Council District'],
            row['Census Tract'],
            row['BIN'],
            row['BBL']
        )

    def migrate_data(self, df, batch_size=50):
        """Migrate data from pandas DataFrame to Cassandra in batches."""
        # Clean the data
        df = self.clean_data(df)

        total_rows = len(df)
        successful_inserts = 0
        failed_inserts = 0

        # Process in batches
        for i in range(0, total_rows, batch_size):
            batch = BatchStatement()
            end_idx = min(i + batch_size, total_rows)

            # Create batch of insertions
            for _, row in df.iloc[i:end_idx].iterrows():
                try:
                    values = self.prepare_row(row)
                    batch.add(self.insert_statement, values)
                except Exception as e:
                    print(f"Error preparing row: {e}")
                    failed_inserts += 1
                    continue

            # Execute batch
            try:
                self.session.execute(batch)
                successful_inserts += len(batch)
                print(f"Processed {successful_inserts}/{total_rows} rows...")
            except Exception as e:
                print(f"Error executing batch: {e}")
                failed_inserts += len(batch)

        return {
            'total_rows': total_rows,
            'successful_inserts': successful_inserts,
            'failed_inserts': failed_inserts
        }

    def close(self):
        """Close the cluster connection."""
        self.cluster.shutdown()


# Example usage
def main():
    print(os.getcwd())
    # Read data from CSV (adjust filename as needed)
    df = pd.read_csv("../data/DOHMH_New_York_City_Restaurant_Inspection_Results_20241030.csv")
    print(len(df))
    df = df.drop_duplicates(subset=['CAMIS'])
    print(len(df))
    df = df[df['BORO'] == 'Manhattan']

    # Initialize migration
    migrator = RestaurantDataMigration()

    try:
        # Perform migration
        results = migrator.migrate_data(df)

        # Print results
        print("\nMigration Results:")
        print(f"Total rows processed: {results['total_rows']}")
        print(f"Successful inserts: {results['successful_inserts']}")
        print(f"Failed inserts: {results['failed_inserts']}")

    finally:
        # Always close the connection
        migrator.close()


if __name__ == "__main__":
    main()