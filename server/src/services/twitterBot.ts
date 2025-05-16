import { TwitterApi } from "twitter-api-v2";
import dotenv from "dotenv";
import { dedupRedis } from "../redis/dedup.client";
import { queueRedis } from "../redis/queue.client";

dotenv.config();

const client = new TwitterApi(process.env.TWITTER_BEARER!);
const roClient = client.readOnly;

const KEYWORDS = ["airdrop", "testnet", "reward", "token", "early access"];

export const searchTweets = async () => {
  try {
    const query = KEYWORDS.map((kw) => `"${kw}"`).join(" OR ");
    const results = await roClient.v2.search(query, {
      max_results: 10,
      expansions: ["author_id"],
      "tweet.fields": ["created_at", "text", "id"],
    });

    const tweets = results.data?.data || [];

    for (const tweet of tweets) {
      const exists = await dedupRedis.get(`tweet:${tweet.id}`);
      if (!exists) {
        await queueRedis.set(`tweet:${tweet.id}`, JSON.stringify(tweet));
        await dedupRedis.set(`tweet:${tweet.id}`, "1"); // ❗️No expiration
        console.log("📤 Queued new tweet for AI:", tweet.text.slice(0, 80));
      }
    }

    return tweets;
  } catch (err) {
    console.error("Twitter fetch error:", err);
    return [];
  }
};
