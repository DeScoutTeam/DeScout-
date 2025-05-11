"use client"

import { useState, useEffect } from "react"
import {
  ArrowRight,
  Bell,
  Github,
  Twitter,
  Search,
  BarChart3,
  Filter,
  Shield,
  Zap,
  ChevronRight,
  Star,
  ExternalLink,
  Sparkles,
  Layers,
  Cpu,
  Lock,
} from "lucide-react"
import { motion } from "framer-motion"
import { blockchains } from "../data/blockchains"
import { trendingProjects } from "../data/projects"
import ProjectCard from "../components/ProjectCard"
import FeatureCard from "../components/FeatureCard"
import StepCard from "../components/StepCard"
import BlockchainCard from "../components/BlockchainCard"
import Navbar from "../components/Navbar"
import Footer from "../components/Footer"
import HeroStats from "../components/HeroStats"

function App() {
  const [email, setEmail] = useState("")
  const [isScrolled, setIsScrolled] = useState(false)

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 50)
    }

    window.addEventListener("scroll", handleScroll)
    return () => window.removeEventListener("scroll", handleScroll)
  }, [])

  const handleSubmit = (e) => {
    e.preventDefault()
    alert(`Thanks for joining! We'll notify you at ${email}`)
    setEmail("")
  }

  return (
    <div className="min-h-screen bg-[#0A0B14] text-white font-sans">
      <div className="fixed top-0 left-0 w-full h-screen pointer-events-none">
        <div className="absolute top-0 left-0 w-full h-full bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-purple-900/20 via-transparent to-transparent"></div>
        <div className="absolute bottom-0 right-0 w-full h-full bg-[radial-gradient(ellipse_at_bottom_right,_var(--tw-gradient-stops))] from-blue-900/20 via-transparent to-transparent"></div>
      </div>

      <Navbar isScrolled={isScrolled} />

      {/* Hero Section */}
      <section className="relative pt-32 pb-20 md:pt-40 md:pb-32 overflow-hidden">
        <div className="container mx-auto px-6 relative z-10">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6 }}
              className="max-w-2xl"
            >
              <div className="inline-flex items-center px-3 py-1 rounded-full bg-purple-500/10 border border-purple-500/20 text-purple-400 text-sm font-medium mb-6">
                <Sparkles className="h-4 w-4 mr-2" />
                <span>AI-Powered Airdrop Intelligence</span>
              </div>
              <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold leading-tight mb-6">
                Never Miss Another{" "}
                <span className="bg-gradient-to-r from-purple-400 to-blue-500 bg-clip-text text-transparent">
                  Airdrop
                </span>{" "}
                Opportunity
              </h1>
              <p className="text-xl text-gray-300 mb-8 leading-relaxed">
                DeScout uses advanced AI to monitor social media, GitHub, on-chain activity, and Discord to detect early
                signals and rank opportunities before they go mainstream.
              </p>

              <div className="flex flex-col sm:flex-row gap-4 mb-10">
                <motion.button
                  whileHover={{ scale: 1.03 }}
                  whileTap={{ scale: 0.98 }}
                  className="bg-gradient-to-r from-purple-600 to-blue-600 px-8 py-4 rounded-xl font-medium hover:shadow-lg hover:shadow-purple-500/20 transition-all flex items-center justify-center"
                >
                  Get Started <ArrowRight className="ml-2 h-5 w-5" />
                </motion.button>
                <motion.button
                  whileHover={{ scale: 1.03 }}
                  whileTap={{ scale: 0.98 }}
                  className="bg-gray-800/80 backdrop-blur-sm border border-gray-700 px-8 py-4 rounded-xl font-medium hover:bg-gray-700/80 transition-all"
                >
                  Watch Demo
                </motion.button>
              </div>

              <HeroStats />
            </motion.div>

            <motion.div
              initial={{ opacity: 0, scale: 0.95 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.7, delay: 0.2 }}
              className="relative"
            >
              <div className="absolute -inset-0.5 bg-gradient-to-r from-purple-500 to-blue-500 rounded-2xl blur-sm opacity-50"></div>
              <div className="relative bg-gray-900 rounded-2xl overflow-hidden border border-gray-800">
                <div className="flex justify-between items-center p-4 border-b border-gray-800 bg-gray-900/90 backdrop-blur-sm">
                  <div className="flex items-center">
                    <div className="h-3 w-3 rounded-full bg-red-500 mr-2"></div>
                    <div className="h-3 w-3 rounded-full bg-yellow-500 mr-2"></div>
                    <div className="h-3 w-3 rounded-full bg-green-500"></div>
                  </div>
                  <div className="text-sm font-medium text-gray-400">DeScout Scanner</div>
                  <div className="flex items-center gap-2">
                    <button className="p-1.5 rounded-md hover:bg-gray-800 transition-colors">
                      <Bell className="h-4 w-4 text-gray-400" />
                    </button>
                    <button className="p-1.5 rounded-md hover:bg-gray-800 transition-colors">
                      <Filter className="h-4 w-4 text-gray-400" />
                    </button>
                  </div>
                </div>

                <div className="p-6">
                  <div className="flex justify-between items-center mb-6">
                    <h3 className="text-xl font-semibold">Top Airdrops</h3>
                    <div className="flex items-center gap-2">
                      <span className="text-xs text-gray-400">Sort by:</span>
                      <select className="text-xs bg-gray-800 border border-gray-700 rounded-md px-2 py-1 focus:outline-none focus:ring-1 focus:ring-purple-500">
                        <option>Probability</option>
                        <option>Deadline</option>
                        <option>Reward Size</option>
                      </select>
                    </div>
                  </div>

                  <div className="space-y-4">
                    {trendingProjects.map((project, index) => (
                      <ProjectCard key={index} project={project} index={index} />
                    ))}
                  </div>

                  <div className="mt-6 pt-4 border-t border-gray-800 flex justify-between items-center">
                    <button className="text-sm text-gray-400 hover:text-white transition-colors">
                      View All Opportunities
                    </button>
                    <div className="flex items-center gap-1">
                      <button className="w-8 h-8 flex items-center justify-center rounded-md bg-gray-800 text-gray-400 hover:bg-gray-700 transition-colors">
                        1
                      </button>
                      <button className="w-8 h-8 flex items-center justify-center rounded-md bg-gray-800 text-gray-400 hover:bg-gray-700 transition-colors">
                        2
                      </button>
                      <button className="w-8 h-8 flex items-center justify-center rounded-md bg-gray-800 text-gray-400 hover:bg-gray-700 transition-colors">
                        3
                      </button>
                      <button className="w-8 h-8 flex items-center justify-center rounded-md bg-gray-800 text-gray-400 hover:bg-gray-700 transition-colors">
                        <ChevronRight className="h-4 w-4" />
                      </button>
                    </div>
                  </div>
                </div>
              </div>

              <div className="absolute -z-10 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[120%] h-[120%] bg-gradient-to-r from-purple-500/10 to-blue-500/10 blur-[100px] rounded-full"></div>
            </motion.div>
          </div>
        </div>

        <div className="absolute top-1/3 left-0 w-full">
          <div className="absolute left-0 w-[30%] h-px bg-gradient-to-r from-transparent via-purple-500/20 to-transparent"></div>
          <div className="absolute right-0 w-[30%] h-px bg-gradient-to-l from-transparent via-blue-500/20 to-transparent"></div>
        </div>
      </section>

      {/* Trusted By Section */}
      <section className="py-12 border-y border-gray-800/50 bg-gray-900/30 backdrop-blur-sm">
        <div className="container mx-auto px-6">
          <div className="text-center mb-8">
            <p className="text-gray-400 text-sm uppercase tracking-wider font-medium">
              Trusted by crypto enthusiasts worldwide
            </p>
          </div>
          <div className="flex flex-wrap justify-center items-center gap-x-12 gap-y-8">
            {["Ethereum Foundation", "Solana Ventures", "Arbitrum DAO", "Optimism Collective", "Base Ecosystem"].map(
              (partner, index) => (
                <div key={index} className="text-gray-500 font-medium text-lg">
                  {partner}
                </div>
              ),
            )}
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section id="features" className="py-24 md:py-32 relative">
        <div className="container mx-auto px-6 relative z-10">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="text-center max-w-3xl mx-auto mb-16"
          >
            <div className="inline-flex items-center px-3 py-1 rounded-full bg-blue-500/10 border border-blue-500/20 text-blue-400 text-sm font-medium mb-4">
              <Cpu className="h-4 w-4 mr-2" />
              <span>Powerful AI Features</span>
            </div>
            <h2 className="text-3xl md:text-4xl font-bold mb-6">Discover Airdrops Before Anyone Else</h2>
            <p className="text-xl text-gray-300">
              DeScout uses advanced machine learning algorithms to analyze patterns and detect airdrop opportunities
              with high precision.
            </p>
          </motion.div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            <FeatureCard
              icon={<Search className="h-8 w-8 text-purple-400" />}
              title="Multi-Source Monitoring"
              description="Scans social media, GitHub, on-chain activity, and Discord for early signals of potential airdrops."
              delay={0.1}
            />
            <FeatureCard
              icon={<BarChart3 className="h-8 w-8 text-blue-400" />}
              title="Probability Ranking"
              description="Projects are ranked based on airdrop probability, deadlines, and requirements using our proprietary algorithm."
              delay={0.2}
            />
            <FeatureCard
              icon={<Filter className="h-8 w-8 text-purple-400" />}
              title="Custom Filters"
              description="Filter opportunities by blockchain, wallet type, action requirements, and minimum expected value."
              delay={0.3}
            />
            <FeatureCard
              icon={<Bell className="h-8 w-8 text-blue-400" />}
              title="Real-time Alerts"
              description="Get notified immediately when high-probability airdrops are detected that match your criteria."
              delay={0.4}
            />
            <FeatureCard
              icon={<Shield className="h-8 w-8 text-purple-400" />}
              title="Risk Assessment"
              description="Each opportunity is analyzed for legitimacy and potential risks to protect you from scams."
              delay={0.5}
            />
            <FeatureCard
              icon={<Zap className="h-8 w-8 text-blue-400" />}
              title="Action Automation"
              description="Automate required actions to qualify for airdrops with a single click through our secure wallet integration."
              delay={0.6}
            />
          </div>
        </div>

        <div className="absolute top-1/2 left-0 w-full h-px bg-gradient-to-r from-transparent via-purple-500/10 to-transparent"></div>
        <div className="absolute top-1/2 right-0 w-full h-px bg-gradient-to-l from-transparent via-blue-500/10 to-transparent"></div>
      </section>

      {/* How It Works */}
      <section id="how-it-works" className="py-24 md:py-32 bg-gray-900/30 relative">
        <div className="container mx-auto px-6 relative z-10">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="text-center max-w-3xl mx-auto mb-16"
          >
            <div className="inline-flex items-center px-3 py-1 rounded-full bg-purple-500/10 border border-purple-500/20 text-purple-400 text-sm font-medium mb-4">
              <Layers className="h-4 w-4 mr-2" />
              <span>Simple Process</span>
            </div>
            <h2 className="text-3xl md:text-4xl font-bold mb-6">How DeScout Works</h2>
            <p className="text-xl text-gray-300">
              Our AI-powered platform does the heavy lifting so you can focus on the most promising opportunities.
            </p>
          </motion.div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 relative">
            <div className="absolute top-1/2 left-0 w-full h-0.5 bg-gradient-to-r from-purple-500/30 via-blue-500/30 to-purple-500/30 transform -translate-y-1/2 hidden md:block"></div>

            <StepCard
              step="01"
              title="Data Collection"
              description="Our AI constantly monitors social media, GitHub, Discord, and on-chain activity for signals of potential airdrops."
              delay={0.1}
            />
            <StepCard
              step="02"
              title="Analysis & Ranking"
              description="Projects are analyzed and ranked based on multiple factors to determine airdrop probability and potential value."
              delay={0.3}
            />
            <StepCard
              step="03"
              title="Action & Tracking"
              description="Complete required actions and track your progress toward qualifying for airdrops with our automated tools."
              delay={0.5}
            />
          </div>

          <div className="mt-16 bg-gradient-to-r from-purple-900/20 via-gray-800/40 to-blue-900/20 rounded-2xl p-8 border border-gray-800">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-8 items-center">
              <div>
                <h3 className="text-2xl font-bold mb-4">Ready to start finding airdrops?</h3>
                <p className="text-gray-300 mb-6">
                  Join thousands of alpha hunters who are already using DeScout to discover early airdrop opportunities.
                </p>
                <motion.button
                  whileHover={{ scale: 1.03 }}
                  whileTap={{ scale: 0.98 }}
                  className="bg-gradient-to-r from-purple-600 to-blue-600 px-6 py-3 rounded-xl font-medium hover:shadow-lg hover:shadow-purple-500/20 transition-all flex items-center"
                >
                  Start Scanning Now <ArrowRight className="ml-2 h-5 w-5" />
                </motion.button>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div className="bg-gray-800/60 backdrop-blur-sm rounded-xl p-4 border border-gray-700">
                  <div className="text-3xl font-bold text-purple-400 mb-2">150+</div>
                  <div className="text-gray-300">Airdrops Found</div>
                </div>
                <div className="bg-gray-800/60 backdrop-blur-sm rounded-xl p-4 border border-gray-700">
                  <div className="text-3xl font-bold text-blue-400 mb-2">$2.5M+</div>
                  <div className="text-gray-300">User Earnings</div>
                </div>
                <div className="bg-gray-800/60 backdrop-blur-sm rounded-xl p-4 border border-gray-700">
                  <div className="text-3xl font-bold text-purple-400 mb-2">24/7</div>
                  <div className="text-gray-300">Monitoring</div>
                </div>
                <div className="bg-gray-800/60 backdrop-blur-sm rounded-xl p-4 border border-gray-700">
                  <div className="text-3xl font-bold text-blue-400 mb-2">10K+</div>
                  <div className="text-gray-300">Active Users</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Supported Blockchains */}
      <section id="blockchains" className="py-24 md:py-32 relative">
        <div className="container mx-auto px-6 relative z-10">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="text-center max-w-3xl mx-auto mb-16"
          >
            <div className="inline-flex items-center px-3 py-1 rounded-full bg-blue-500/10 border border-blue-500/20 text-blue-400 text-sm font-medium mb-4">
              <Lock className="h-4 w-4 mr-2" />
              <span>Multi-Chain Support</span>
            </div>
            <h2 className="text-3xl md:text-4xl font-bold mb-6">Supported Blockchains</h2>
            <p className="text-xl text-gray-300">
              DeScout monitors airdrop opportunities across all major blockchain ecosystems.
            </p>
          </motion.div>

          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6">
            {blockchains.map((blockchain, index) => (
              <BlockchainCard key={index} blockchain={blockchain} index={index} />
            ))}
          </div>
        </div>
      </section>

      {/* Community */}
      <section id="community" className="py-24 md:py-32 bg-gray-900/30 relative">
        <div className="container mx-auto px-6 relative z-10">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="text-center max-w-3xl mx-auto mb-16"
          >
            <div className="inline-flex items-center px-3 py-1 rounded-full bg-purple-500/10 border border-purple-500/20 text-purple-400 text-sm font-medium mb-4">
              <Star className="h-4 w-4 mr-2" />
              <span>Join Our Community</span>
            </div>
            <h2 className="text-3xl md:text-4xl font-bold mb-6">DeScout DAO</h2>
            <p className="text-xl text-gray-300">
              Share hidden alpha or private drops in exchange for tokens or exclusive access.
            </p>
          </motion.div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div className="bg-gray-800/40 backdrop-blur-sm rounded-2xl p-8 border border-gray-700 hover:border-purple-500/50 transition-colors">
              <h3 className="text-2xl font-bold mb-4">Community Benefits</h3>
              <ul className="space-y-4 mb-6">
                {[
                  "Access to exclusive airdrops not available to the public",
                  "Share alpha with the community and earn rewards",
                  "Participate in governance and shape the future of DeScout",
                  "Connect with other alpha hunters and share strategies",
                  "Early access to new features and tools",
                ].map((benefit, index) => (
                  <li key={index} className="flex items-start">
                    <div className="h-6 w-6 rounded-full bg-gradient-to-r from-purple-500 to-blue-500 flex items-center justify-center flex-shrink-0 mt-0.5 mr-3">
                      <svg
                        className="h-3 w-3 text-white"
                        viewBox="0 0 24 24"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                      >
                        <path
                          d="M5 12L10 17L20 7"
                          stroke="currentColor"
                          strokeWidth="3"
                          strokeLinecap="round"
                          strokeLinejoin="round"
                        />
                      </svg>
                    </div>
                    <span className="text-gray-300">{benefit}</span>
                  </li>
                ))}
              </ul>
              <div className="flex flex-wrap gap-4">
                <motion.button
                  whileHover={{ scale: 1.03 }}
                  whileTap={{ scale: 0.98 }}
                  className="flex items-center bg-[#1DA1F2] px-6 py-3 rounded-xl font-medium hover:opacity-90 transition-opacity"
                >
                  <Twitter className="mr-2 h-5 w-5" /> Twitter
                </motion.button>
                <motion.button
                  whileHover={{ scale: 1.03 }}
                  whileTap={{ scale: 0.98 }}
                  className="flex items-center bg-[#5865F2] px-6 py-3 rounded-xl font-medium hover:opacity-90 transition-opacity"
                >
                  <svg className="mr-2 h-5 w-5" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M20.317 4.3698a19.7913 19.7913 0 00-4.8851-1.5152.0741.0741 0 00-.0785.0371c-.211.3753-.4447.8648-.6083 1.2495-1.8447-.2762-3.68-.2762-5.4868 0-.1636-.3847-.4058-.874-.6177-1.2495a.077.077 0 00-.0785-.037 19.7363 19.7363 0 00-4.8852 1.515.0699.0699 0 00-.0321.0277C.5334 9.0458-.319 13.5799.0992 18.0578a.0824.0824 0 00.0312.0561c2.0528 1.5076 4.0413 2.4228 5.9929 3.0294a.0777.0777 0 00.0842-.0276c.4616-.6304.8731-1.2952 1.226-1.9942a.076.076 0 00-.0416-.1057c-.6528-.2476-1.2743-.5495-1.8722-.8923a.077.077 0 01-.0076-.1277c.1258-.0943.2517-.1923.3718-.2914a.0743.0743 0 01.0776-.0105c3.9278 1.7933 8.18 1.7933 12.0614 0a.0739.0739 0 01.0785.0095c.1202.099.246.1981.3728.2924a.077.077 0 01-.0066.1276 12.2986 12.2986 0 01-1.873.8914.0766.0766 0 00-.0407.1067c.3604.698.7719 1.3628 1.225 1.9932a.076.076 0 00.0842.0286c1.961-.6067 3.9495-1.5219 6.0023-3.0294a.077.077 0 00.0313-.0552c.5004-5.177-.8382-9.6739-3.5485-13.6604a.061.061 0 00-.0312-.0286z" />
                  </svg>
                  Discord
                </motion.button>
                <motion.button
                  whileHover={{ scale: 1.03 }}
                  whileTap={{ scale: 0.98 }}
                  className="flex items-center bg-gray-700 px-6 py-3 rounded-xl font-medium hover:bg-gray-600 transition-colors"
                >
                  <Github className="mr-2 h-5 w-5" /> GitHub
                </motion.button>
              </div>
            </div>

            <div className="bg-gray-800/40 backdrop-blur-sm rounded-2xl p-8 border border-gray-700 hover:border-blue-500/50 transition-colors">
              <h3 className="text-2xl font-bold mb-4">Stay Updated</h3>
              <p className="text-gray-300 mb-6">
                Subscribe to our newsletter to get the latest airdrop opportunities and platform updates delivered
                directly to your inbox.
              </p>
              <form onSubmit={handleSubmit} className="space-y-4">
                <div>
                  <label htmlFor="email" className="block text-sm font-medium text-gray-400 mb-1">
                    Email Address
                  </label>
                  <input
                    type="email"
                    id="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    placeholder="Enter your email"
                    className="w-full px-4 py-3 bg-gray-700/60 backdrop-blur-sm rounded-xl border border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all"
                    required
                  />
                </div>
                <div className="flex items-start mb-4">
                  <div className="flex items-center h-5">
                    <input
                      id="terms"
                      type="checkbox"
                      className="w-4 h-4 bg-gray-700 border-gray-600 rounded focus:ring-purple-500"
                      required
                    />
                  </div>
                  <label htmlFor="terms" className="ml-2 text-sm text-gray-400">
                    I agree to receive emails about DeScout updates and opportunities
                  </label>
                </div>
                <motion.button
                  whileHover={{ scale: 1.02 }}
                  whileTap={{ scale: 0.98 }}
                  type="submit"
                  className="w-full bg-gradient-to-r from-purple-600 to-blue-600 px-6 py-3 rounded-xl font-medium hover:shadow-lg hover:shadow-purple-500/20 transition-all"
                >
                  Subscribe to Updates
                </motion.button>
              </form>

              <div className="mt-6 pt-6 border-t border-gray-700">
                <div className="flex items-center justify-between">
                  <div className="text-sm text-gray-400">Already a member?</div>
                  <a href="#" className="text-sm text-purple-400 hover:text-purple-300 transition-colors">
                    Sign in to your account
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-24 md:py-32 relative overflow-hidden">
        <div className="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-5"></div>
        <div className="absolute top-0 left-0 w-full h-full bg-gradient-to-br from-purple-900/20 via-transparent to-blue-900/20"></div>

        <div className="container mx-auto px-6 relative z-10">
          <div className="max-w-4xl mx-auto text-center">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6 }}
            >
              <h2 className="text-4xl md:text-5xl font-bold mb-6">Ready to Find Your Next Airdrop?</h2>
              <p className="text-xl text-gray-300 mb-10 max-w-2xl mx-auto">
                Join thousands of alpha hunters who are already using DeScout to discover early airdrop opportunities.
              </p>

              <div className="flex flex-col sm:flex-row justify-center gap-4">
                <motion.button
                  whileHover={{ scale: 1.03 }}
                  whileTap={{ scale: 0.98 }}
                  className="bg-gradient-to-r from-purple-600 to-blue-600 px-8 py-4 rounded-xl font-medium hover:shadow-lg hover:shadow-purple-500/20 transition-all flex items-center justify-center"
                >
                  Get Started Now <ArrowRight className="ml-2 h-5 w-5" />
                </motion.button>
                <motion.button
                  whileHover={{ scale: 1.03 }}
                  whileTap={{ scale: 0.98 }}
                  className="bg-white text-gray-900 px-8 py-4 rounded-xl font-medium hover:bg-gray-100 transition-colors flex items-center justify-center"
                >
                  <ExternalLink className="mr-2 h-5 w-5" /> View Live Demo
                </motion.button>
              </div>
            </motion.div>
          </div>
        </div>
      </section>

      <Footer />
    </div>
  )
}

export default App
