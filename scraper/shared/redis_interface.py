from redis import Redis
import os

def create_redis_client():
    print(f"REDIS {os.getenv('REDIS_HOST')}: {os.getenv('REDIS_PORT')}")
    redis_host = os.getenv('REDIS_HOST', 'localhost')
    redis_port = int(os.getenv('REDIS_PORT', 6379))
    client = Redis(host=redis_host, port=redis_port, db=0)
    return client


def create_redis_queue(client: Redis, queue_name: str, db_num = 1):
    def push(item):
        client.select(db_num)
        client.lpush(queue_name, item)

    def pop():
        client.select(db_num)
        client.lpop(queue_name)
        return client.lpop(queue_name)

    def length() -> int:
        client.select(db_num)
        return client.llen(queue_name)

    def is_empty():
        var = length() == 0
        return var

    return push, pop, length, is_empty


def create_document_db_interface(client: Redis):
    def put_item(item_name, data):
        client.select(0)
        client.set(item_name, data)

    def delete_item(item_name):
        client.select(0)
        client.delete(item_name)

    def fetch_item(item_name):
        client.select(0)
        if client.exists(item_name):
            return client.get(item_name)
        else:
            return None
    return put_item, delete_item, fetch_item

