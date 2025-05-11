"use client"

import { motion } from "framer-motion"
import { Clock, ArrowUpRight } from "lucide-react"

const ProjectCard = ({ project, index }) => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3, delay: index * 0.1 }}
      className="bg-gray-800/60 backdrop-blur-sm rounded-xl p-4 hover:bg-gray-800 transition-colors border border-gray-700 hover:border-gray-600"
    >
      <div className="flex justify-between items-center">
        <div className="flex items-center">
          <div className="w-10 h-10 rounded-lg overflow-hidden mr-3 flex-shrink-0">
            <img
              src={project.logo || "/placeholder.svg?height=40&width=40"}
              alt={project.name}
              className="w-full h-full object-cover"
            />
          </div>
          <div>
            <div className="flex items-center">
              <h4 className="font-medium">{project.name}</h4>
              {project.verified && (
                <div className="ml-1.5 bg-blue-500/20 text-blue-400 text-xs px-1.5 py-0.5 rounded">Verified</div>
              )}
            </div>
            <div className="flex items-center text-sm text-gray-400">
              <Clock className="h-3 w-3 mr-1" />
              <span>{project.deadline}</span>
            </div>
          </div>
        </div>
        <div className="flex items-center">
          <div
            className={`px-2 py-1 rounded text-xs font-medium ${
              project.probability === "High"
                ? "bg-green-500/20 text-green-400"
                : project.probability === "Medium"
                  ? "bg-yellow-500/20 text-yellow-400"
                  : "bg-gray-500/20 text-gray-400"
            }`}
          >
            {project.probability}
          </div>
          <button className="ml-2 p-1.5 rounded-md hover:bg-gray-700 transition-colors">
            <ArrowUpRight className="h-4 w-4 text-gray-400" />
          </button>
        </div>
      </div>

      {project.tasks && (
        <div className="mt-3 pt-3 border-t border-gray-700">
          <div className="flex justify-between items-center text-xs text-gray-400">
            <div>
              Required Tasks: {project.completedTasks}/{project.totalTasks}
            </div>
            <div className="flex items-center">
              <div className="w-20 h-1.5 bg-gray-700 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-purple-500 to-blue-500 rounded-full"
                  style={{ width: `${(project.completedTasks / project.totalTasks) * 100}%` }}
                ></div>
              </div>
            </div>
          </div>
        </div>
      )}
    </motion.div>
  )
}

export default ProjectCard
