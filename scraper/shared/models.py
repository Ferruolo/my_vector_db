from typing import List
from pydantic import BaseModel, HttpUrl, Field

class DisplayName(BaseModel):
    text: str
    languageCode: str

class LocationData(BaseModel):
    types: List[str]
    formattedAddress: str
    websiteUri: str
    displayName: DisplayName

    class Config:
        extra = "forbid"  # This will raise an error if there are extra fields in the input data
