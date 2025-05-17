from upstash_redis import Redis
import json
from src.ai.config import REDIS_URL, REDIS_TOKEN

redis_client = Redis(url=REDIS_URL, token=REDIS_TOKEN)


def fetch_all_tweet_ids():
    print(redis_client.keys("*"))
    return redis_client.keys("*")


def get_tweet_by_id(tweet_id: str) -> dict:
    try:
        value = redis_client.get(tweet_id)

        print(value)
        if not value:
            return None

        return json.loads(value)
    except Exception as e:
        print(f"❌ Error fetching tweet {tweet_id}: {e}")
        return None

def fetch_all_keys_batch(cursor=0, batch_size=1000):
    result = redis_client.scan(cursor=cursor, count=batch_size)
    next_cursor = result[0]
    keys = result[1]
    return next_cursor, keys

def fetch_all_tweets_batch(batch_size=100):
    cursor = 0
    all_tweets = []

    while True:
        cursor, keys = redis_client.scan(cursor=cursor, match="*", count=batch_size)
        print(keys)
        if not keys:
            if cursor == 0:
                break
            continue

        values = redis_client.mget(*keys)

        print(values)
        for key, value in zip(keys, values):
            if value:
                try:
                    tweet = json.loads(value)
                    print(tweet)
                    all_tweets.append((key, tweet))
                except json.JSONDecodeError:
                    print(f"❌ Failed to decode JSON for key {key}")
        if cursor == 0:
            break

    return all_tweets


def periodic_fetch(interval_minutes=1):
    while True:
        print(f"Fetching all tweets from Redis...")
        tweets = fetch_all_tweets()
        print(f"Fetched {len(tweets)} tweets.")

        print(f"Sleeping for {interval_minutes} minutes...")
        time.sleep(interval_minutes * 60)

def delete_tweet(tweet_id: str):
    redis_client.delete(tweet_id)