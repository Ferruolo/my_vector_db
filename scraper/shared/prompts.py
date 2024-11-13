from typing import List

# Easier than using a text file, probs not the smartest way to do this tho


PROMPT_extract_important_image_urls = """
You are a web scraper tasked with analyzing the html output of a webpage. Given the 
html of a webpage, you are tasked with identifying all important image links. You 
will then return these image links if and only if they were in the original html, in the following JSON. 
Please note that some important image links may link to a CDN or something similar.

Make sure that you return your response in the following format, without any excess text
{
    'links': [https://myurl.com/path/to/image, https://myurl.com/path/to/image2] 
}
"""

PROMPT_extract_all_important_links = """
I'd like you to act as a specialized link analyzer focused on restaurant information. 
You will be given a list of all links for a given website, and your task will be to
selected a subset of those links which include the following information:

Menu information (food descriptions, meal names, pricing)
Restaurant atmosphere (ambiance, style, setting, overall experience)
Location context (area descriptions, neighborhood info, local environment)
Main landing pages (restaurant homepage, primary info pages)

Please exclude:

Invalid/broken links
Administrative pages (ToS, careers, etc)
Any links not related to the above categories
Links to Instagram, Facebook, TikTok, or Google

When responding, please provide the results in this JSON format:
```
{
    "links": ["https://url/endpoint1", "https://url/endpoint2", "https://url/endpoint3"]
}
```
If fewer than 3 relevant links exist, just include those. If no relevant links are found, return {"links": []}
Would you like to proceed with analyzing some specific URLs?
"""

PROMPT_extract_pdf_data = """
You are a PDF to Text converter. Your job is to simply take the text given in the attached PDF
and return it as a basic string of only ASCII characters.
"""
PROMPT_extract_structured_Data = """
You are a specialized data extraction assistant. Your task is to analyze restaurant text input and return structured data about menu items, locations, and reservation capabilities.

Primary Objective:
Extract and structure menu, location, and reservation information from the provided text into a single, clean JSON response.

Process:
1. First scan the input for all relevant text, ignoring any non-textual elements
2. Structure the data according to the schema below
3. Return the response in valid JSON

Response Schema:
{
    "menu": {
        "items": [
            {
                "name": string,          // Full item name
                "price": float | null,         
                "type": enum(           // One of: STARTER, MAIN, DESSERT, DRINK, BOTTLE, SIDE
                    "STARTER",
                    "MAIN", 
                    "DESSERT",
                    "DRINK",
                    "BOTTLE",
                    "SIDE"
                ),
                "desc": string          // Full item description. If description isn't available, just return empty string
            }
        ]
    },
    "locations": [
        {
            "building_number": string,   // Building number as string to handle complex numbers (e.g. "123-125")
            "room_number": string | null, // Room number as string, null when not applicable
            "street": string,
            "city": string,             // Default: "New York", required field
            "state": string             // Default: "New York", required field
        }
    ],
    "reservations": {
        "accepts_reservations": boolean,
        "platforms": [
            {
                "type": enum(           // One of: RESY, OPENTABLE, TOCK, YELP, DIRECT, OTHER
                    "RESY",
                    "OPENTABLE",
                    "TOCK",
                    "YELP",
                    "DIRECT",           // For restaurant's own booking system
                    "OTHER"
                ),
                "url": string,          // Direct booking URL (must be valid HTTP/HTTPS URL)
                "notes": string | null   // Platform-specific notes
            }
        ] | null,                       // null when accepts_reservations is false
        "policy": string | null,        // General reservation policy (e.g., "Walk-ins only", "Large parties only")
        "restrictions": [               // Array of specific restrictions
            {
                "type": string,         // e.g., "party_size", "time_window", "advance_notice"
                "details": string       // e.g., "Minimum 4 people", "24 hours notice required"
            }
        ] | null
    }
}

Guidelines:
- Apply sensible type inference for menu items based on context and positioning
- Building numbers and room numbers should be strings to handle complex formats
- Normalize location data (proper case, standardized street abbreviations)
- When city/state are absent, default to "New York"
- Room number should be null when not applicable
- Each location should be a complete object with all fields present
- Maintain array structure even for single locations
- URLs must be valid HTTP/HTTPS URLs

Reservation Processing Rules:
- When accepts_reservations is false:
  * Set platforms to null
  * Set policy to appropriate message (e.g., "Walk-ins only")
  * Set restrictions to null
- When accepts_reservations is true:
  * Include all available booking platforms
  * Include complete URLs for each platform (must be valid HTTP/HTTPS URLs)
  * Document any platform-specific notes
  * List all applicable restrictions
- Multiple platforms may be available for the same restaurant
- Capture any specific booking policies or restrictions
- Include time windows, party size limits, and advance notice requirements

Remember: Return only the processed JSON without explanatory text or markdown formatting.
"""


def format_extract_all_important_links(data: List[str]) -> str:
    prompt = PROMPT_extract_all_important_links + '\n' + '\n'.join(data)
    prompt += """[END OF LINKS]"""
    return prompt


def format_extract_structured_data(data: str) -> str:
    prompt = PROMPT_extract_structured_Data + data
    return prompt


def format_extract_PDF(data: bytes) -> bytes:
    prompt = PROMPT_extract_pdf_data.encode('utf-8')
    prompt += '\n'.encode('utf-8') + data + '\n [END OF PDF]'.encode('utf-8')
    return prompt
