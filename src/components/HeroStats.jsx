"use client"

import { motion } from "framer-motion"

const HeroStats = () => {
  const stats = [
    { value: "150+", label: "Airdrops Found" },
    { value: "$2.5M+", label: "User Earnings" },
    { value: "10K+", label: "Active Users" },
  ]

  return (
    <div className="flex flex-wrap gap-6 mt-2">
      {stats.map((stat, index) => (
        <motion.div
          key={index}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.6 + index * 0.1 }}
          className="flex items-center"
        >
          <div className="mr-2 h-8 w-0.5 bg-gradient-to-b from-purple-500 to-blue-500 rounded-full"></div>
          <div>
            <div className="text-xl font-bold">{stat.value}</div>
            <div className="text-sm text-gray-400">{stat.label}</div>
          </div>
        </motion.div>
      ))}
    </div>
  )
}

export default HeroStats
