import { Router } from "express";
import {
  getAllProjects,
  addProject,
} from "../controllers/projects";

const router = Router();

router.get("/", getAllProjects);
router.post("/", addProject);

export default router;
