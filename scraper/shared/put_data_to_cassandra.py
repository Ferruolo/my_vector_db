from typing import List, Tuple, Optional, Dict
from uuid import UUID, uuid4
from shared.models import Business
from cassandra.cluster import Session


class CassandraInsertionError(Exception):
    pass


def insert_business(
        session: Session,
        item_id: UUID,
        biz_name: str,
        yelp_id: str,
        supports_pickup: Optional[bool] = None,
        supports_delivery: Optional[bool] = None,
        yelp_rating: Optional[float] = None,
        phone_number: Optional[str] = None,
        website_url: Optional[str] = None,
) -> None:
    print(
        f"{item_id} | {biz_name} | {yelp_id} | {supports_pickup} | {supports_delivery} | {yelp_rating}  | {phone_number} | {website_url} ")
    try:

        type_validator = Business(id=str(item_id),
                                  biz_name=biz_name,
                                  yelp_id=yelp_id,
                                  supports_pickup=supports_pickup,
                                  supports_delivery=supports_delivery,
                                  yelp_rating=yelp_rating,
                                  phone_number=phone_number,
                                  website_url="'" + website_url + "'")

        query = """
        INSERT INTO businesses (
            id, yelp_id, biz_name, supports_pickup, supports_delivery,
            yelp_rating, phone_number, website_url
        ) VALUES (%s, %s, %s, %s, %s, %s, %s, %s)
        """

        session.execute(query, (
            type_validator.id, type_validator.yelp_id, type_validator.biz_name,
            type_validator.supports_pickup, type_validator.supports_delivery,
            type_validator.yelp_rating,
            type_validator.phone_number, type_validator.website_url
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
        ) VALUES (%s, %s, %s, %s, %s, %s)
        """

        session.execute(query, (
            uuid4(), business_id, item_name, item_type, item_desc, item_price
        ))
    except Exception as e:
        raise CassandraInsertionError(f"Failed to insert menu item: {str(e)}")


def insert_text_data(
        session: Session,
        business_id: UUID,
        text_selection: str,
        embedding: List[float]
) -> None:
    if len(embedding) != 4096:
        raise ValueError(f"Embedding must be 4096 dimensions, got {len(embedding)}")

    try:
        query = """
        INSERT INTO text_data (
            entry_id, business_id, text_selection, embedding
        ) VALUES (%s, %s, %s, %s)
        """

        session.execute(query, (
            uuid4(), business_id, text_selection, embedding
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
        ) VALUES (%s, %s, %s, %s, %s)
        """

        session.execute(query, (
            uuid4(), business_id, open_time, close_time, day_of_week
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
        ) VALUES (%s, %s, %s)
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
        ) VALUES (?, ?, ?, ?)
        """

        session.execute(query, (
            uuid4(), location_id, data, embedding
        ))
    except Exception as e:
        raise CassandraInsertionError(f"Failed to insert location data: {str(e)}")


def insert_business_location(
        session: Session,
        location_id: UUID,
        biz_id: UUID,
        latitude: float,
        longitude: float,
        building_number: str,
        roomNumber: str,
        street: str,
        city: str,
        state: str
) -> None:
    query = """
        INSERT INTO biz_locations 
        (location_id, biz_id, latitude, longitude, building_number, room_number, street, city, state)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    """

    prepared = session.prepare(query)

    try:
        session.execute(
            prepared,
            (location_id, biz_id, latitude, longitude, building_number, roomNumber, street, city, state)
        )
    except Exception as e:
        raise CassandraInsertionError(f"Failed to insert business location: {str(e)}")


def insert_reservation_data(session: Session, business_id: str, reservation_data: Dict) -> None:
    try:
        basic_info_query = """
            INSERT INTO business_reservations (
                business_id,
                accepts_reservations,
                reservation_policy
            ) VALUES (%s, %s, %s)
        """
        session.execute(
            basic_info_query,
            (
                business_id,
                reservation_data["accepts_reservations"],
                reservation_data.get("policy")
            )
        )

        if reservation_data["accepts_reservations"] and reservation_data.get("platforms"):
            platform_query = """
                INSERT INTO business_reservation_platforms (
                    business_id,
                    platform_type,
                    url,
                    notes
                ) VALUES (%s, %s, %s, %s)
            """
            for platform in reservation_data["platforms"]:
                session.execute(
                    platform_query,
                    (
                        business_id,
                        platform["type"],
                        platform["url"],
                        platform.get("notes")
                    )
                )

        if reservation_data.get("restrictions"):
            restriction_query = """
                INSERT INTO business_reservation_restrictions (
                    business_id,
                    restriction_type,
                    restriction_details
                ) VALUES (%s, %s, %s)
            """
            for restriction in reservation_data["restrictions"]:
                session.execute(
                    restriction_query,
                    (
                        business_id,
                        restriction["type"],
                        restriction["details"]
                    )
                )
    except Exception as e:
        cleanup_queries = [
            "DELETE FROM business_reservations WHERE business_id = ?",
            "DELETE FROM business_reservation_platforms WHERE business_id = ?",
            "DELETE FROM business_reservation_restrictions WHERE business_id = ?"
        ]

        for query in cleanup_queries:
            try:
                session.execute(query, [business_id])
            except:
                pass

        raise CassandraInsertionError(f"Failed to insert reservation data: {str(e)}")
