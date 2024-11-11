import math

FNV_PRIME_32 = 16777619
FNV_OFFSET_32 = 2166136261


def fnv1a_32(data: bytes) -> int:
    hash_value = FNV_OFFSET_32
    for byte in data:
        hash_value ^= byte
        hash_value *= FNV_PRIME_32
    return hash_value & 0xFFFFFFFF  # Ensure 32-bit unsigned int


def chained_fnv1a(data: str, k: int) -> int:
    result = fnv1a_32(data.encode('utf-8'))
    for _ in range(k - 1):
        result = fnv1a_32(result.to_bytes(4, byteorder='little', signed=False))
    return result


class BloomFilter:
    def __init__(self, capacity: int, expected_entries: int):
        self.capacity = capacity + 7
        self.expected_entries = expected_entries
        self.bit_arr = bytearray(self.capacity)
        self.k = math.floor((self.capacity / self.expected_entries) * math.log(2))

    def add_item(self, url: str):
        index = chained_fnv1a(url, self.k) % self.capacity
        self.bit_arr[index // 8] |= 1 << (index % 8)

    def clear_item(self, url: str):
        index = chained_fnv1a(url, self.k) % self.capacity
        self.bit_arr[index // 8] &= ~(1 << (index % 8))

    def get_item(self, url: str):
        index = chained_fnv1a(url, self.k) % self.capacity

        return bool(self.bit_arr[index // 8] & (1 << (index % 8)))