import { Router } from "express";
import { getLatestAirdrops } from "../db/actions/airdrop.actions";
import { fetchTweetsHandler } from "../controllers/twitter";

const router = Router();

router.get("/fetch", fetchTweetsHandler);

router.get("/", async (_req, res) => {
  const drops = await getLatestAirdrops(20);
  res.json(drops);
});

export default router;
