import { motion } from "framer-motion"

const BlockchainCard = ({ blockchain, index }) => {
  return (
    <motion.div 
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true }}
      transition={{ duration: 0.3, delay: index * 0.05 }}
      whileHover={{ y: -5 }}
      className="bg-gray-800/40 backdrop-blur-sm rounded-xl p-5 flex flex-col items-center justify-center border border-gray-700 hover:border-purple-500/30 transition-all"
    >
      <img 
        src={blockchain.logo || "/placeholder.svg?height=40&width=40"} 
        alt={blockchain.name} 
        className="h-12 mb-4"
      />
      <span className="text-center font-medium">{blockchain.name}</span>
      {blockchain.isNew && (
        <span className="mt-2 px-2 py-0.5 bg-purple-500/20 text-purple-400 text-xs rounded-full">New</span>
      )}
    </motion.div>
  )
}

export default BlockchainCard
