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



PROMPT_extract_all_important_links = """<s>[INST] <<SYS>>
You are a specialized link analyzer focused on extracting relevant restaurant information links. Your responses must be precise and contain only the requested JSON format.

TASK DEFINITION:
Analyze provided URLs and extract only links containing essential restaurant information.

REQUIRED DATA CATEGORIES:
1. MENU INFORMATION
   - Food descriptions
   - Meal names
   - Pricing

2. RESTAURANT ATMOSPHERE
   - Ambiance descriptions
   - Style and setting
   - Overall experience

3. LOCATION CONTEXT
   - Area description
   - Neighborhood characteristics
   - Local environment

4. LANDING PAGES
   - Main restaurant homepage
   - Primary information hub
   - Core service pages

EXCLUDE:
- Invalid or non-functioning links
- Administrative pages (terms of service, careers, etc)
- Any links not related to the categories above

JSON OUTPUT FORMAT:

"{
    "links": ["https://url/endpoint1", "https://url/endpoint2", "https://url/endpoint3"]
}"

If no relevant links found, return: {"links": []}
<</SYS>>

ANALYZE THESE LINKS:
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
                "price": string,         // Price including currency symbol
                "type": enum(           // One of: STARTER, MAIN, DESSERT, DRINK, BOTTLE, SIDE
                    "STARTER",
                    "MAIN", 
                    "DESSERT",
                    "DRINK",
                    "BOTTLE",
                    "SIDE"
                ),
                "desc": string          // Full item description
            }
        ]
    },
    "locations": [
        {
            "building_number": number,
            "room_number": number | null,
            "street": string,
            "city": string,             // Default: "New York"
            "state": string             // Default: "New York"
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
                "url": string,          // Direct booking URL
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
- Preserve exact price formatting including currency symbols
- Normalize location data (proper case, standardized street abbreviations)
- When city/state are absent, default to "New York"
- Room number should be null when not applicable
- Each location should be a complete object with all fields present
- Maintain array structure even for single locations

Reservation Processing Rules:
- When accepts_reservations is false:
  * Set platforms to null
  * Set policy to appropriate message (e.g., "Walk-ins only")
  * Set restrictions to null
- When accepts_reservations is true:
  * Include all available booking platforms
  * Include complete URLs for each platform
  * Document any platform-specific notes
  * List all applicable restrictions
- Multiple platforms may be available for the same restaurant
- Capture any specific booking policies or restrictions
- Include time windows, party size limits, and advance notice requirements

Remember: Return only the processed JSON without explanatory text or markdown formatting.
"""

def format_extract_all_important_links(data: List[str]) -> str:
    prompt = PROMPT_extract_all_important_links + '\n'.join(data)
    prompt += """ [/INST]</s>"""
    return prompt


def format_extract_structered_data(data: str) -> str:
    prompt = PROMPT_extract_structured_Data + data
    return prompt
