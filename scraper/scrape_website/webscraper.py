from typing import Optional, Dict, List, Any
import asyncio
from pyppeteer import launch


class Puppeteer:
    def __init__(self, headless: bool = True, user_agent: Optional[str] = None):
        self.browser = None
        self.page = None
        self.headless = headless
        self.user_agent = user_agent or 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'

    async def start(self):
        self.browser = await launch(headless=self.headless)
        self.page = await self.browser.newPage()
        await self.page.setUserAgent(self.user_agent)

    async def stop(self):
        if self.browser:
            await self.browser.close()

    async def goto(self, url: str, wait_until: str = 'networkidle0'):
        if not self.page:
            await self.start()
        await self.page.goto(url, {'waitUntil': wait_until})

    async def get_text(self, selector: str) -> str:
        element = await self.page.querySelector(selector)
        if element:
            return await self.page.evaluate('(element) => element.textContent', element)
        return ''

    async def get_texts(self, selector: str) -> List[str]:
        elements = await self.page.querySelectorAll(selector)
        texts = []
        for element in elements:
            text = await self.page.evaluate('(element) => element.textContent', element)
            texts.append(text)
        return texts

    async def click(self, selector: str):
        await self.page.click(selector)

    async def type(self, selector: str, text: str):
        await self.page.type(selector, text)

    async def wait_for_selector(self, selector: str, timeout: int = 30000):
        await self.page.waitForSelector(selector, {'timeout': timeout})

    async def screenshot(self, path: str):
        await self.page.screenshot({'path': path})

    async def get_cookies(self) -> List[Dict[str, Any]]:
        return await self.page.cookies()

    async def set_cookies(self, cookies: List[Dict[str, Any]]):
        await self.page.setCookie(*cookies)

    async def evaluate(self, js_code: str):
        return await self.page.evaluate(js_code)

    async def get_all_links(self) -> List[Dict[str, str]]:
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
        return links

    async def get_all_images(self) -> List[str]:
        images = await self.page.evaluate('''
            () => Array.from(document.getElementsByTagName('img')).map(img => ({
                src: img.src
            }))
        ''')

        # Filter out data URLs and empty sources
        return [img['src'] for img in images if img['src'] and not img['src'].startswith('data:')]
