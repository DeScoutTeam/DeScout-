import { Redis } from "@upstash/redis";

import dotenv from "dotenv"

dotenv.config();

export const queueRedis = new Redis({
  url: process.env.UPSTASH_REDIS_QUEUE_URL!,
  token: process.env.UPSTASH_REDIS_QUEUE_TOKEN!,
});
