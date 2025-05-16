type Project = {
    name: string;
    description: string;
    tags: string[];
  };
  
  const projects: Project[] = [];
  
  export const getProjectsFromDB = async () => projects;
  
  export const saveProjectToDB = async (project: Project) => {
    projects.push(project);
    return project;
  };
  