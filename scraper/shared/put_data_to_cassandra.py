from uuid import UUID
from cassandra.cluster import Session
from datetime import datetime
from cassandra.util import datetime_from_uuid1
import logging
from typing import List, Tuple, Optional, Dict, Any


class CassandraInsertionError(Exception):
    pass


def insert_business(
        session: Session,
        id: UUID,
        biz_name: str,
        yelp_id: Optional[str] = None,
        supports_pickup: Optional[bool] = None,
        supports_delivery: Optional[bool] = None,
        yelp_rating: Optional[float] = None,
        latitude: Optional[float] = None,
        longitude: Optional[float] = None,
        price_magnitude: Optional[int] = None,
        phone_number: Optional[str] = None,
        website_url: Optional[str] = None,
        reservation_link: Optional[str] = None,
        building_number: Optional[int] = None,
        street: Optional[str] = None,
        city: Optional[str] = None,
        state: Optional[str] = None
) -> None:
    try:
        query = """
        INSERT INTO businesses (
            id, yelp_id, biz_name, supports_pickup, supports_delivery,
            yelp_rating, latitude, longitude, price_magnitude,
            phone_number, website_url, reservation_link,
            building_number, street, city, state
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """

        session.execute(query, (
            id, yelp_id, biz_name, supports_pickup, supports_delivery,
            yelp_rating, latitude, longitude, price_magnitude,
            phone_number, website_url, reservation_link,
            building_number, street, city, state
        ))
    except Exception as e:
        raise CassandraInsertionError(f"Failed to insert business record: {str(e)}")


def insert_menu_item(
        session: Session,
        business_id: UUID,
        item_name: str,
        item_type: str,
        item_price: float,
        item_desc: Optional[str] = None
) -> None:
    valid_types = {'STARTER', 'MAIN', 'DESSERT', 'DRINK', 'BOTTLE', 'SIDE'}
    if item_type not in valid_types:
        raise ValueError(f"item_type must be one of {valid_types}")

    try:
        query = """
        INSERT INTO menu_data (
            item_id, business_id, item_name, item_type, item_desc, item_price
        ) VALUES (now(), ?, ?, ?, ?, ?)
        """

        session.execute(query, (
            business_id, item_name, item_type, item_desc, item_price
        ))
    except Exception as e:
        raise CassandraInsertionError(f"Failed to insert menu item: {str(e)}")


def insert_text_data(
        session: Session,
        business_id: UUID,
        text_selection: str,
        source: str,
        embedding: List[float]
) -> None:
    if len(embedding) != 4096:
        raise ValueError("Embedding must be 4096 dimensions")

    try:
        query = """
        INSERT INTO text_data (
            entry_id, business_id, text_selection, source, embedding
        ) VALUES (now(), ?, ?, ?, ?)
        """

        session.execute(query, (
            business_id, text_selection, source, embedding
        ))
    except Exception as e:
        raise CassandraInsertionError(f"Failed to insert text data: {str(e)}")


def insert_opening_hours(
        session: Session,
        business_id: UUID,
        open_time: int,
        close_time: int,
        day_of_week: int
) -> None:
    if not (0 <= open_time <= 2359 and 0 <= close_time <= 2359):
        raise ValueError("Times must be in 24hr format between 0000 and 2359")

    if not (1 <= day_of_week <= 7):
        raise ValueError("day_of_week must be between 1 and 7 (Monday=1)")

    try:
        query = """
        INSERT INTO opening_data (
            entry_id, business_id, open_time, close_time, day_of_week
        ) VALUES (now(), ?, ?, ?, ?)
        """

        session.execute(query, (
            business_id, open_time, close_time, day_of_week
        ))
    except Exception as e:
        raise CassandraInsertionError(f"Failed to insert opening hours: {str(e)}")


def insert_location(
        session: Session,
        location_id: UUID,
        location_name: str,
        boundaries: List[Tuple[float, float]]
) -> None:
    try:
        query = """
        INSERT INTO locations (
            location_id, location_name, boundaries
        ) VALUES (?, ?, ?)
        """

        session.execute(query, (
            location_id, location_name, boundaries
        ))
    except Exception as e:
        raise CassandraInsertionError(f"Failed to insert location: {str(e)}")


def insert_location_data(
        session: Session,
        location_id: UUID,
        data: str,
        embedding: List[float]
) -> None:
    if len(embedding) != 4096:
        raise ValueError("Embedding must be 4096 dimensions")

    try:
        query = """
        INSERT INTO location_data (
            entry_id, location_id, data, embedding
        ) VALUES (now(), ?, ?, ?)
        """

        session.execute(query, (
            location_id, data, embedding
        ))
    except Exception as e:
        raise CassandraInsertionError(f"Failed to insert location data: {str(e)}")