import AirdropModel from "../models/airdrop.model";

export const findAirdropByTweetId = async (tweetId: string) => {
  return await AirdropModel.findOne({ tweetId });
};

export const createAirdrop = async (data: {
  tweetId: string;
  text: string;
  authorId: string;
  createdAt: string | Date;
}) => {
  return await AirdropModel.create({
    tweetId: data.tweetId,
    text: data.text,
    authorId: data.authorId,
    createdAt: data.createdAt,
  });
};

export const getLatestAirdrops = async (limit = 20) => {
  return await AirdropModel.find().sort({ savedAt: -1 }).limit(limit);
};
