import { Request, Response } from "express";
import { getProjectsFromDB, saveProjectToDB } from "../db/projects";

export const getAllProjects = async (_req: Request, res: Response) => {
  const projects = await getProjectsFromDB();
  res.json(projects);
};

export const addProject = async (req: Request, res: Response) => {
  const { name, description, tags } = req.body;
  const newProject = await saveProjectToDB({ name, description, tags });
  res.status(201).json(newProject);
};
