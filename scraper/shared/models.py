from typing import List, Optional, Literal
from pydantic import BaseModel, Field, HttpUrl
from uuid import UUID


class DisplayName(BaseModel):
    text: str
    languageCode: str


class MenuItem(BaseModel):
    name: str
    price: Optional[float | None]
    type: Literal["STARTER", "MAIN", "DESSERT", "DRINK", "BOTTLE", "SIDE"]
    desc: str


class Menu(BaseModel):
    items: List[MenuItem]


class Location(BaseModel):
    building_number: Optional[str] = None
    room_number: Optional[str] = None
    street: str
    city: str = "New York"
    state: str = "New York"


class ReservationRestriction(BaseModel):
    type: str
    details: str


class ReservationPlatform(BaseModel):
    type: Literal["RESY", "OPENTABLE", "TOCK", "YELP", "DIRECT", "OTHER"]
    url: HttpUrl
    notes: Optional[str] = None


class Reservations(BaseModel):
    accepts_reservations: bool
    platforms: Optional[List[ReservationPlatform]] = Field(
        None,
        description="List of reservation platforms. Null when accepts_reservations is false"
    )
    policy: Optional[str] = Field(
        None,
        description="General reservation policy (e.g., 'Walk-ins only', 'Large parties only')"
    )
    restrictions: Optional[List[ReservationRestriction]] = None


class Restaurant(BaseModel):
    menu: Menu
    locations: List[Location]
    reservations: Reservations


class Business(BaseModel):
    id: UUID
    yelp_id: Optional[str] = None
    biz_name: str
    supports_pickup: bool
    supports_delivery: bool
    yelp_rating: Optional[float] = Field(None, ge=0, le=5)
    phone_number: Optional[str] = None
    website_url: Optional[str] = None
