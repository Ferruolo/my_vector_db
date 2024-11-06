import re
from urllib.parse import urlparse
from typing import Optional



def drop_repeated_newline_regex(my_str: str) -> str:
    return re.sub(r'\n\s*\n', '\n', my_str)

def extract_base_url(url: str) -> Optional[str]:
    try:
        parsed = urlparse(url)
        if parsed.scheme and parsed.netloc:
            return f"{parsed.scheme}://{parsed.netloc}"
        return None
    except Exception:
        return None


def is_toast_tab_link(url: str) -> bool:
    if url.lower() == "https://www.toasttab.com":
        return True
    else:
        return False


def is_internal_link(url: str, base_site: str) -> bool:
    url_base = extract_base_url(url)
    if not url_base:
        return True
    if url_base.lower() == "https://www.toasttab.com":
        return True
    site_base = extract_base_url(base_site)
    if not site_base:
        return False
    return url_base.lower() == site_base.lower()
