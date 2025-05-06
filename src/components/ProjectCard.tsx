import React from "react";
import { Airdrop } from "../types";

type Props = {
  project: Airdrop;
};

const ProjectCard: React.FC<Props> = ({ project }) => (
  <div className="bg-gray-900 rounded-xl p-4 border border-gray-800 hover:border-purple-600 transition-all">
    <h3 className="text-lg font-bold text-white">{project.name}</h3>
    <p className="text-gray-400">{project.description}</p>
    <div className="flex justify-between mt-2 text-sm text-gray-500">
      <span>⛓ {project.chain}</span>
      <span>🕒 {project.deadline}</span>
      <span>🧩 {project.action}</span>
    </div>
  </div>
);

export default ProjectCard;
