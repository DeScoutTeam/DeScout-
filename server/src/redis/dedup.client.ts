import { Redis } from "@upstash/redis";
import dotenv from "dotenv"

dotenv.config();

export const dedupRedis = new Redis({
  url: process.env.UPSTASH_REDIS_DEDUP_URL!,
  token: process.env.UPSTASH_REDIS_DEDUP_TOKEN!,
});
