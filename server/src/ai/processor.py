import json
import time
import threading
from concurrent.futures import ThreadPoolExecutor, as_completed
from src.ai.llama_client import query_llm
from src.ai.redis_client import fetch_all_tweets_batch, delete_tweet
from src.ai.mongodb_client import save_processed_post

MAX_WORKERS = 10  # tune this, but keep in mind llama-cpp-python memory limits
llama_lock = threading.Lock()  # serialize llama calls


def process_single_tweet(tweet_id, tweet):
    if not tweet:
        return

    try:
        with llama_lock:
            result = query_llm(tweet.get("text", ""))

        doc = {
            "tweetId": tweet.get("id") or tweet.get("id_str"),
            "text": tweet.get("text"),
            "authorId": tweet.get("author_id") or tweet.get("user", {}).get("id_str"),
            "createdAt": tweet.get("created_at"),
            "deepness": result.get("deepness", 1),
            "keywords": result.get("keywords", []),
            "tokenName": result.get("tokenName", ""),
            "mentionedUsers": result.get("mentionedUsers", []),
            "links": result.get("links", []),
        }
        save_processed_post(doc)
        delete_tweet(tweet_id)
    except Exception as e:
        print(f"❌ Error processing tweet {tweet_id}: {e}")


def process_queue(interval_seconds=60):
    while True:
        try:
            print("Fetching tweets batch...")
            tweet_items = fetch_all_tweets_batch()
            print(f"Fetched {len(tweet_items)} tweets.")

            with ThreadPoolExecutor(max_workers=MAX_WORKERS) as executor:
                futures = [executor.submit(process_single_tweet, tweet_id, tweet) for tweet_id, tweet in tweet_items]

                for future in as_completed(futures):
                    try:
                        future.result()
                    except Exception as e:
                        print(f"❌ Worker error: {e}")

        except Exception as e:
            print(f"❌ Error fetching tweets: {e}")

        print(f"Sleeping for {interval_seconds} seconds...")
        time.sleep(interval_seconds)
