from typing import List, Optional
from pydantic import BaseModel, HttpUrl, Field

class DisplayName(BaseModel):
    text: str
    languageCode: str

class LocationData(BaseModel):
    types: Optional[List[str]] = None
    formattedAddress: Optional[str] = None
    websiteUri: str
    displayName: Optional[DisplayName] = None
    # class Config:
    #     extra = "forbid"  # This will raise an error if there are extra fields in the input data
