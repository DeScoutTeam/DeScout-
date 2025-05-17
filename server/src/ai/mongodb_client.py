from pymongo import MongoClient
from src.ai.config import MONGO_URI, MONGO_DB_NAME, MONGO_COLLECTION_NAME

client = MongoClient(MONGO_URI)
db = client[MONGO_DB_NAME]
collection = db[MONGO_COLLECTION_NAME]


def save_processed_post(post_doc: dict):
    collection.insert_one(post_doc)
