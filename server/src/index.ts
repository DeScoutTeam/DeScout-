import express from "express";
import cors from "cors";
import dotenv from "dotenv";
import projectRoutes from "./routes/projects";
import twitterRoutes from "./routes/twitter";
import { connectDB } from "./db/connection";

dotenv.config();
const app = express();
const PORT = process.env.PORT || 4000;

app.use(cors());
app.use(express.json());

const start = async () => {
  await connectDB();
};

start();

app.use("/api/projects", projectRoutes);

app.get("/", (_req, res) => {
  res.send("AI Airdrop Scanner Backend is running.");
});

app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});

app.use("/api/airdrop-alerts", twitterRoutes);
