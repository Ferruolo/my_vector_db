from typing import Optional, Dict, List, Any
from bs4 import BeautifulSoup
from playwright.async_api import async_playwright, Browser, Page, TimeoutError
from contextlib import asynccontextmanager
import logging

class Playwright:
    def __init__(self, headless: bool = True, user_agent: Optional[str] = None):
        self.browser: Optional[Browser] = None
        self.page: Optional[Page] = None
        self.headless = headless
        self.user_agent = user_agent
        self.logger = logging.getLogger(__name__)
        self.playwright = None

    async def __aenter__(self):
        await self.start()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.stop()

    async def start(self):
        try:
            self.playwright = await async_playwright().start()
            self.browser = await self.playwright.chromium.launch(
                headless=self.headless,
                args=['--no-sandbox', '--disable-setuid-sandbox']
            )
            self.page = await self.browser.new_page()
            if self.user_agent:
                await self.page.set_extra_http_headers({"User-Agent": self.user_agent})
            # No need to set default timeout as Playwright handles this differently
            # No need for stealth as Playwright has better built-in evasion
        except Exception as e:
            self.logger.error(f"Failed to start browser: {str(e)}")
            if self.browser:
                await self.browser.close()
            if self.playwright:
                await self.playwright.stop()
            raise

    async def stop(self):
        try:
            if self.page:
                await self.page.close()
            if self.browser:
                await self.browser.close()
            if self.playwright:
                await self.playwright.stop()
        except Exception as e:
            self.logger.error(f"Error during cleanup: {str(e)}")
            raise

    async def _ensure_page(self):
        if not self.page:
            await self.start()

    async def goto(self, url: str, wait_until: str = 'domcontentloaded'):
        try:
            await self._ensure_page()
            # Convert Puppeteer's wait_until to Playwright's wait_until
            wait_until_map = {
                'domcontentloaded': 'domcontentloaded',
                'load': 'load',
                'networkidle0': 'networkidle',
                'networkidle2': 'networkidle'
            }
            playwright_wait_until = wait_until_map.get(wait_until, 'domcontentloaded')
            await self.page.goto(url, wait_until=playwright_wait_until)
        except TimeoutError as e:
            self.logger.warning(f"Navigation timeout for URL: {url}")
            raise
        except Exception as e:
            self.logger.error(f"Failed to navigate to {url}: {str(e)}")
            raise

    async def get_text(self, selector: str) -> str:
        try:
            await self._ensure_page()
            element = await self.page.query_selector(selector)
            if element:
                return await element.text_content() or ''
            return ''
        except Exception as e:
            self.logger.error(f"Error getting text for selector {selector}: {str(e)}")
            raise

    async def get_texts(self, selector: str) -> List[str]:
        try:
            await self._ensure_page()
            elements = await self.page.query_selector_all(selector)
            return [await element.text_content() or '' for element in elements]
        except Exception as e:
            self.logger.error(f"Error getting texts for selector {selector}: {str(e)}")
            raise

    async def click(self, selector: str):
        try:
            await self._ensure_page()
            await self.page.click(selector)
        except Exception as e:
            self.logger.error(f"Error clicking selector {selector}: {str(e)}")
            raise

    async def type(self, selector: str, text: str):
        try:
            await self._ensure_page()
            await self.page.type(selector, text)
        except Exception as e:
            self.logger.error(f"Error typing into selector {selector}: {str(e)}")
            raise

    async def wait_for_selector(self, selector: str, timeout: int = 30000):
        try:
            await self._ensure_page()
            await self.page.wait_for_selector(selector, timeout=timeout)
        except TimeoutError as e:
            self.logger.warning(f"Timeout waiting for selector: {selector}")
            raise
        except Exception as e:
            self.logger.error(f"Error waiting for selector {selector}: {str(e)}")
            raise

    async def screenshot(self, path: str):
        try:
            await self._ensure_page()
            await self.page.screenshot(path=path)
        except Exception as e:
            self.logger.error(f"Error taking screenshot: {str(e)}")
            raise

    async def get_cookies(self) -> List[Dict[str, Any]]:
        try:
            await self._ensure_page()
            return await self.page.context.cookies()
        except Exception as e:
            self.logger.error(f"Error getting cookies: {str(e)}")
            raise

    async def set_cookies(self, cookies: List[Dict[str, Any]]):
        try:
            await self._ensure_page()
            await self.page.context.add_cookies(cookies)
        except Exception as e:
            self.logger.error(f"Error setting cookies: {str(e)}")
            raise

    async def evaluate(self, js_code: str):
        try:
            await self._ensure_page()
            return await self.page.evaluate(js_code)
        except Exception as e:
            self.logger.error(f"Error evaluating JavaScript: {str(e)}")
            raise

    async def get_all_links(self) -> List[str]:
        try:
            await self._ensure_page()
            links = await self.page.evaluate('''
                () => {
                    return Array.from(document.getElementsByTagName('a')).map(link => ({
                        href: link.href,
                        text: link.textContent.trim(),
                        title: link.title || '',
                        rel: link.rel || '',
                        isVisible: 
                            window.getComputedStyle(link).display !== 'none' &&
                            window.getComputedStyle(link).visibility !== 'hidden' &&
                            link.offsetParent !== null
                    }));
                }
            ''')
            return [link['href'] for link in links]
        except Exception as e:
            self.logger.error(f"Error getting all links: {str(e)}")
            raise

    async def get_all_images(self) -> List[str]:
        try:
            await self._ensure_page()
            images = await self.page.evaluate('''
                () => Array.from(document.getElementsByTagName('img')).map(img => ({
                    src: img.src
                }))
            ''')
            return [img['src'] for img in images if img['src'] and not img['src'].startswith('data:')]
        except Exception as e:
            self.logger.error(f"Error getting all images: {str(e)}")
            raise

    async def get_page_soup(self) -> BeautifulSoup:
        try:
            await self._ensure_page()
            content = await self.page.content()
            return BeautifulSoup(content, 'html.parser')
        except Exception as e:
            self.logger.error(f"Error getting page soup: {str(e)}")
            raise