from collections import deque

from shared.bloomfilter import BloomFilter


class UniqueSearchContainer:
    def __init__(self, capacity: int, expected_entries: int, useDFS: bool):
        self.bloom_filter = BloomFilter(capacity, expected_entries)
        self.search_queue = deque()
        self.useDFS = useDFS

    def push(self, key: str):
        if not self.bloom_filter.get_item(key):
            self.bloom_filter.add_item(key)
            self.search_queue.append(key)  # Appends to RIGHT end

    def pop(self):
        if self.useDFS:
            # pops from right end, emulates stack behavior for depth first search
            self.search_queue.pop()
        else:
            # pops from left end of stack, emulates queue behavior for breadth first search
            self.search_queue.popleft()

    def is_empty(self):
        return len(self.search_queue) == 0

    def get_len(self):
        return len(self.search_queue)
