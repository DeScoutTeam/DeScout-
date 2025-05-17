import os
from dotenv import load_dotenv

load_dotenv()

REDIS_URL = os.getenv("UPSTASH_REDIS_QUEUE_URL")
REDIS_TOKEN = os.getenv("UPSTASH_REDIS_QUEUE_TOKEN")
MONGO_URI = os.getenv("MONGODB_URI")
LLAMA_MODEL_PATH = os.getenv("LLAMA_MODEL_PATH", "./models/llama.bin")

REDIS_QUEUE_NAME = "DeScout_queue"
MONGO_DB_NAME = "test"
MONGO_COLLECTION_NAME = "airdrops"
