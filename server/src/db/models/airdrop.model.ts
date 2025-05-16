import mongoose from "mongoose";

const AirdropSchema = new mongoose.Schema({
  tweetId: { type: String, required: true, unique: true },
  text: String,
  authorId: String,
  createdAt: Date,
  savedAt: { type: Date, default: Date.now },

  deepness: { type: Number, default: 1 },
  keywords: [String],
  tokenName: String,
  mentionedUsers: [String],
  links: [String],
});

const AirdropModel = mongoose.model("Airdrop", AirdropSchema);

export default AirdropModel;
