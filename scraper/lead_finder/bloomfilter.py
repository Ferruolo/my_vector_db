FNV_PRIME_32 = 16777619
FNV_OFFSET_32 = 2166136261


def fnv1a_32(data: int) -> int:
    hash_value = FNV_OFFSET_32
    for _ in range(4):  # Process 4 bytes (32 bits)
        byte = data & 0xFF
        hash_value ^= byte
        hash_value *= FNV_PRIME_32
        data >>= 8
    return hash_value & 0xFFFFFFFF  # Ensure 32-bit unsigned int


def int_to_bytes(value: int) -> bytes:
    return value.to_bytes(4, byteorder='little', signed=False)


def bytes_to_int(value: bytes) -> int:
    return int.from_bytes(value, byteorder='little', signed=False)


def chained_fnv1a(data: str, k: int) -> int:
    # Initial conversion of string to integer
    result = bytes_to_int(data.encode()[:4].ljust(4, b'\0'))
    for _ in range(k):
        result = fnv1a_32(result)
    return result
