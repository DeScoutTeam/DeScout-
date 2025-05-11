"use client"

import { motion } from "framer-motion"

const StepCard = ({ step, title, description, delay = 0 }) => {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true }}
      transition={{ duration: 0.5, delay }}
      className="relative"
    >
      <div className="absolute -top-3 left-1/2 transform -translate-x-1/2 w-6 h-6 rounded-full bg-gradient-to-r from-purple-500 to-blue-500 flex items-center justify-center z-10 hidden md:flex">
        <div className="w-2 h-2 rounded-full bg-white"></div>
      </div>

      <div className="bg-gray-800/40 backdrop-blur-sm rounded-xl p-6 border border-gray-700 hover:border-purple-500/30 transition-all h-full">
        <div className="text-4xl font-bold bg-gradient-to-r from-purple-400 to-blue-500 bg-clip-text text-transparent mb-4">
          {step}
        </div>
        <h3 className="text-xl font-semibold mb-3">{title}</h3>
        <p className="text-gray-300">{description}</p>
      </div>
    </motion.div>
  )
}

export default StepCard
