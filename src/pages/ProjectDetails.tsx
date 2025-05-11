import React from "react";
import { Airdrop } from "../types";

type Props = {
  project: Airdrop | null;
};

const ProjectDetails: React.FC<Props> = ({ project }) => {
  if (!project) return <div className="text-gray-500">Оберіть проект для деталей</div>;

  return (
    <div className="p-4 bg-gray-800 rounded-xl text-white space-y-2">
      <h2 className="text-xl font-bold">{project.name}</h2>
      <p>{project.description}</p>
      <p>⛓ Ланцюг: {project.chain}</p>
      <p>🕒 Дедлайн: {project.deadline}</p>
      <p>🧩 Необхідні дії: {project.action}</p>
    </div>
  );
};

export default ProjectDetails;
