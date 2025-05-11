import { motion } from "framer-motion"

const FeatureCard = ({ icon, title, description, delay = 0 }) => {
  return (
    <motion.div 
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true }}
      transition={{ duration: 0.5, delay }}
      className="bg-gray-800/40 backdrop-blur-sm rounded-xl p-6 border border-gray-700 hover:border-purple-500/30 transition-all group"
    >
      <div className="bg-gradient-to-br from-gray-700 to-gray-800 rounded-lg w-14 h-14 flex items-center justify-center mb-5 group-hover:from-purple-900/50 group-hover:to-blue-900/50 transition-all">
        {icon}
      </div>
      <h3 className="text-xl font-semibold mb-3">{title}</h3>
      <p className="text-gray-300">{description}</p>
    </motion.div>
  )
}

export default FeatureCard
