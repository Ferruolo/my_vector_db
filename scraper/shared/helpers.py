import re
import time
from functools import wraps
import random
from urllib.parse import urlparse
from typing import Optional
import json


def drop_repeated_newline_regex(my_str: str) -> str:
    return re.sub(r'\n\s*\n', '\n', my_str)


def strip_white_space(my_str: str) -> str:
    return '\n'.join([s.strip() for s in my_str.split('\n')])


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


def extract_json(text: str):
    stack, start = [], -1

    for i, c in enumerate(text):
        if c in '{[':
            if c == '{' and not stack: start = i
            stack.append(c)
        elif c in '}]':
            if stack and ((c == '}' and stack[-1] == '{') or (c == ']' and stack[-1] == '[')):
                stack.pop()
                if not stack and start >= 0:
                    try:
                        return json.loads(text[start:i + 1])
                    except:
                        continue

    raise ValueError("No valid JSON found")


def drop_duplicate_sentences(data: str) -> str:
    container = set()
    for line in data.split('\n'):
        for sent in line.split('.'):
            container.add(sent)
    merged = list(container)
    return '\n'.join(merged)


def retry_with_backoff(max_retries=5, base_delay=1, max_delay=32, jitter=True):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            retries = 0
            while retries < max_retries:
                try:
                    return func(*args, **kwargs)
                except Exception as e:
                    retries += 1
                    if retries == max_retries:
                        raise e

                    # Calculate delay with exponential backoff
                    delay = min(base_delay * (2 ** (retries - 1)), max_delay)

                    # Add jitter if enabled
                    if jitter:
                        delay = delay * (0.5 + random.random())

                    print(f"Attempt {retries} failed. Retrying in {delay:.2f} seconds...")
                    time.sleep(delay)
            return None

        return wrapper

    return decorator
