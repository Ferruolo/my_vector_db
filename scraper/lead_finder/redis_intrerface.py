from charset_normalizer.cli import cli_detect
from redis import Redis


def create_redis_client():
    client = Redis(host='localhost', port=6379, db=0)
    return client


def create_redis_queue(client: Redis, queue_name: str, db_num: 1):
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
        var = length() is 0
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

