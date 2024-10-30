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
                inspection_date text,
                dba text,
                boro text,
                building text,
                street text,
                zipcode decimal,
                phone text,
                cuisine_description text,
                action text,
                violation_code text,
                violation_description text,
                critical_flag text,
                score decimal,
                grade text,
                grade_date text,
                record_date text,
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
        df = df.copy()

        # Convert text columns to string type, replacing nan with None
        text_columns = ['DBA', 'BORO', 'BUILDING', 'STREET', 'CUISINE DESCRIPTION',
                       'ACTION', 'VIOLATION CODE', 'VIOLATION DESCRIPTION',
                       'CRITICAL FLAG', 'GRADE', 'INSPECTION TYPE', "PHONE", 'INSPECTION DATE', 'GRADE DATE', 'RECORD DATE']
        for col in text_columns:
            if col in df.columns:
                df[col] = df[col].astype(str).replace('nan', None)

        # Handle numeric columns
        numeric_columns = {
            'CAMIS': ('integer', 0),
            'ZIPCODE': ('integer', None),
            'SCORE': ('float', None),
            'Latitude': ('float', None),
            'Longitude': ('float', None),
            'Community Board': ('integer', None),
            'Council District': ('integer', None),
            'Census Tract': ('integer', None),
            'BIN': ('integer', None),
            'BBL': ('integer', None)
        }

        for col, (type_name, default_value) in numeric_columns.items():
            if col in df.columns:
                def safe_convert(val):
                    if pd.isna(val) or val == '' or str(val).lower() == 'nan':
                        return 0
                    try:
                        if type_name == 'integer':
                            # Convert to float first to handle decimal strings
                            return int(float(val))
                        return float(val)
                    except (ValueError, TypeError):
                        return 0

                df[col] = df[col].apply(safe_convert)

        return df

    def prepare_row(self, row):
        """Convert a pandas row to a tuple of values for Cassandra insertion."""
        # Convert empty strings and 'nan' to None for text fields
        def clean_text(val):
            if pd.isna(val) or str(val).lower() == 'nan' or str(val).strip() == '':
                return None
            return str(val)

        return (
            row['CAMIS'],
            row['INSPECTION DATE'],
            clean_text(row['DBA']),
            clean_text(row['BORO']),
            clean_text(row['BUILDING']),
            clean_text(row['STREET']),
            row['ZIPCODE'],
            row['PHONE'],
            clean_text(row['CUISINE DESCRIPTION']),
            clean_text(row['ACTION']),
            clean_text(row['VIOLATION CODE']),
            clean_text(row['VIOLATION DESCRIPTION']),
            clean_text(row['CRITICAL FLAG']),
            row['SCORE'],
            clean_text(row['GRADE']),
            row['GRADE DATE'],
            row['RECORD DATE'],
            clean_text(row['INSPECTION TYPE']),
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
            current_batch_rows = []

            # Create batch of insertions
            for _, row in df.iloc[i:end_idx].iterrows():
                try:
                    values = self.prepare_row(row)
                    batch.add(self.insert_statement, values)
                    current_batch_rows.append(values)
                except Exception as e:
                    print(f"Error preparing row: {e}")
                    # print(f"Problematic row data: {row}")
                    failed_inserts += 1
                    continue

            # Execute batch
            if current_batch_rows:
                try:
                    self.session.execute(batch)
                    successful_inserts += len(current_batch_rows)
                    print(f"Processed {successful_inserts}/{total_rows} rows...")
                except Exception as e:
                    print(f"Error executing batch: {e}")
                    print(f"First row in failed batch: {current_batch_rows[0]}")
                    failed_inserts += len(current_batch_rows)

        return {
            'total_rows': total_rows,
            'successful_inserts': successful_inserts,
            'failed_inserts': failed_inserts
        }

    def close(self):
        """Close the cluster connection."""
        self.cluster.shutdown()

def main():
    df = pd.read_csv("../data/DOHMH_New_York_City_Restaurant_Inspection_Results_20241030.csv")
    df = df.drop_duplicates(subset=['CAMIS'])
    df = df[df['BORO'] == 'Manhattan']

    migrator = RestaurantDataMigration()
    try:
        results = migrator.migrate_data(df)
        print("\nMigration Results:")
        print(f"Total rows processed: {results['total_rows']}")
        print(f"Successful inserts: {results['successful_inserts']}")
        print(f"Failed inserts: {results['failed_inserts']}")
    finally:
        migrator.close()

if __name__ == "__main__":
    main()