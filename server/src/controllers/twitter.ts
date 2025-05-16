import { Request, Response } from "express";
import { searchTweets } from "../services/twitterBot";

export const fetchTweetsHandler = async (_req: Request, res: Response) => {
  try {
    const tweets = await searchTweets();
    res.json({ success: true, tweets });
  } catch (error) {
    console.error("Error fetching tweets:", error);
    res.status(500).json({ success: false, error: "Internal Server Error" });
  }
};
