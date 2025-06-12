import { Button } from "@/components/ui/button"
import { ArrowRight } from "lucide-react"

export default function DeScoutLanding() {
  return (
    <div className="min-h-screen bg-black relative overflow-hidden">

      <div className="absolute inset-0">
        <div className="absolute top-[7%] left-[11%] w-1 h-1 bg-white rounded-full opacity-90" />
        <div className="absolute top-[11%] left-[7%] w-0.5 h-0.5 bg-white rounded-full opacity-70" />
        <div className="absolute top-[5%] left-[19%] w-1 h-1 bg-white rounded-full opacity-80" />
        <div className="absolute top-[13%] left-[24%] w-0.5 h-0.5 bg-white rounded-full opacity-60" />
        <div className="absolute top-[9%] left-[33%] w-1 h-1 bg-white rounded-full opacity-85" />
        <div className="absolute top-[16%] left-[74%] w-0.5 h-0.5 bg-white rounded-full opacity-65" />
        <div className="absolute top-[7%] left-[84%] w-1 h-1 bg-white rounded-full opacity-75" />
        <div className="absolute top-[12%] left-[89%] w-0.5 h-0.5 bg-white rounded-full opacity-55" />
      </div>
      
      <div className="absolute w-full h-full flex justify-end">
        <div className="relative h-full w-1/2">
          <div
            className="absolute -top-20 left-1/2 transform -translate-x-1/2"
            style={{
              width: "400px",
              height: "900px",
              background:
                "linear-gradient(to bottom, rgba(59,130,246,0.4) 0%, rgba(29,78,216,0.3) 40%, rgba(29,78,216,0.2) 70%, transparent 100%)",
              clipPath: "polygon(15% 0%, 85% 0%, 100% 100%, 0% 100%)",
            }}
          />

          <div
            className="absolute -top-20 left-1/2 transform -translate-x-1/2"
            style={{
              width: "320px",
              height: "900px",
              background:
                "linear-gradient(to bottom, rgba(147,197,253,0.7) 0%, rgba(59,130,246,0.5) 30%, rgba(29,78,216,0.4) 60%, transparent 100%)",
              clipPath: "polygon(18% 0%, 82% 0%, 98% 100%, 2% 100%)",
            }}
          />

          <div
            className="absolute -top-20 left-1/2 transform -translate-x-1/2"
            style={{
              width: "260px",
              height: "900px",
              background:
                "linear-gradient(to bottom, rgba(255,255,255,0.8) 0%, rgba(147,197,253,0.7) 25%, rgba(59,130,246,0.5) 50%, rgba(29,78,216,0.3) 75%, transparent 100%)",
              clipPath: "polygon(22% 0%, 78% 0%, 95% 100%, 5% 100%)",
            }}
          />

          <div
            className="absolute -top-20 left-1/2 transform -translate-x-1/2"
            style={{
              width: "200px",
              height: "900px",
              background:
                "linear-gradient(to bottom, rgba(255,255,255,0.95) 0%, rgba(147,197,253,0.8) 30%, rgba(59,130,246,0.6) 60%, rgba(29,78,216,0.3) 80%, transparent 100%)",
              clipPath: "polygon(25% 0%, 75% 0%, 92% 100%, 8% 100%)",
            }}
          />

          <div
            className="absolute -top-20 left-1/2 transform -translate-x-1/2"
            style={{
              width: "150px",
              height: "900px",
              background:
                "linear-gradient(to bottom, rgba(255,255,255,1) 0%, rgba(255,255,255,0.9) 20%, rgba(147,197,253,0.7) 50%, rgba(59,130,246,0.4) 80%, transparent 100%)",
              clipPath: "polygon(28% 0%, 72% 0%, 88% 100%, 12% 100%)",
            }}
          />

          <div
            className="absolute -top-20 left-1/2 transform -translate-x-1/2"
            style={{
              width: "100px",
              height: "900px",
              background:
                "linear-gradient(to bottom, rgba(255,255,255,1) 0%, rgba(255,255,255,0.95) 30%, rgba(147,197,253,0.8) 70%, transparent 100%)",
              clipPath: "polygon(32% 0%, 68% 0%, 85% 100%, 15% 100%)",
            }}
          />

          <div className="absolute -top-20 left-1/2 transform -translate-x-1/2 w-[600px] h-[900px] bg-blue-400/10 blur-[120px] pointer-events-none" />

          <div className="absolute -top-20 left-1/2 transform -translate-x-1/2 w-[400px] h-[900px] bg-blue-300/15 blur-[80px] pointer-events-none" />

          <div className="absolute -top-20 left-1/2 transform -translate-x-1/2 w-[250px] h-[900px] bg-white/20 blur-[50px] pointer-events-none" />

          <div className="absolute -top-20 left-1/2 transform -translate-x-1/2 w-[150px] h-[900px] bg-white/30 blur-[30px] pointer-events-none" />

          <div className="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-[300px] h-[200px] bg-gradient-to-t from-blue-600/25 via-blue-400/15 to-transparent rounded-full blur-3xl" />
        </div>
      </div>

      <header className="relative z-20 px-8 py-7">
        <div className="max-w-[1400px] mx-auto flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <div className="w-8 h-8 bg-white rounded-[6px] flex items-center justify-center">
              <span className="text-black font-bold text-[11px] leading-none">🔍</span>
            </div>
            <span className="text-white text-[21px] font-medium tracking-[0.01em]">DeScout</span>
          </div>

          <nav className="hidden md:flex items-center bg-[#0D0D1F]/85 backdrop-blur-xl rounded-full px-1.5 py-1.5 border border-white/[0.06]">
            <a href="#" className="text-white px-[22px] py-[10px] rounded-full relative text-[14px] font-medium">
              Scanner
              <div className="absolute bottom-[6px] left-1/2 transform -translate-x-1/2 w-[22px] h-[2px] bg-blue-500 rounded-full" />
            </a>
            <a
              href="#"
              className="text-white/65 hover:text-white px-[22px] py-[10px] transition-colors text-[14px] font-normal"
            >
              Filters
            </a>
            <a
              href="#"
              className="text-white/65 hover:text-white px-[22px] py-[10px] transition-colors text-[14px] font-normal"
            >
              Rankings
            </a>
            <a
              href="#"
              className="text-white/65 hover:text-white px-[22px] py-[10px] transition-colors text-[14px] font-normal"
            >
              DAO
            </a>
            <a
              href="#"
              className="text-white/65 hover:text-white px-[22px] py-[10px] transition-colors text-[14px] font-normal"
            >
              Alpha
            </a>
          </nav>

          <Button className="bg-[#0D0D1F]/85 border border-white/[0.08] text-white hover:bg-[#0D0D1F] backdrop-blur-xl px-[22px] py-[10px] rounded-[8px] text-[14px] font-medium h-auto">
            Join DAO
          </Button>
        </div>
      </header>

      <main className="relative z-10 px-8 pt-[72px] pb-[88px]">
        <div className="max-w-[1400px] mx-auto">
          <div className="grid lg:grid-cols-2 gap-[120px] items-center">
            <div className="space-y-[36px]">
              <div className="inline-flex items-center px-[14px] py-[6px] bg-transparent border border-white/[0.12] rounded-full text-white/75 text-[13px] font-light tracking-[0.02em] leading-none">
                [ AI-Powered Alpha Detection ]
              </div>

              <div className="space-y-[20px]">
                <h1 className="text-[64px] font-bold text-white leading-[1.05] tracking-[-0.02em] font-sans">
                  Hunt Airdrops
                  <br />
                  Before Everyone
                </h1>

                <p className="text-[18px] text-white/55 leading-[1.6] font-light max-w-[420px] pt-[8px] tracking-[0.005em]">
                  AI-powered scanner monitoring social media,
                  <br />
                  GitHub, and on-chain activity for early signals
                </p>
              </div>

              <div className="relative inline-block pt-[16px]">
                <div className="absolute -inset-[8px] bg-blue-600/35 rounded-[18px] blur-[12px]"></div>
                <Button className="relative bg-blue-600 hover:bg-blue-700 text-white px-[24px] py-[16px] text-[15px] rounded-[14px] font-medium border border-blue-500/40 h-auto shadow-lg">
                  Start Scanning
                  <ArrowRight className="ml-[8px] h-[16px] w-[16px]" strokeWidth={2} />
                </Button>
              </div>
            </div>

            <div className="relative h-[500px] flex items-center justify-center">
              <div className="w-64 h-64 bg-blue-500/20 rounded-full blur-3xl absolute"></div>

              <div className="relative space-y-6">
                <div className="w-64 h-40 bg-gradient-to-br from-slate-900/90 to-slate-800/90 backdrop-blur-xl rounded-2xl shadow-2xl transform rotate-6 border border-white/10 p-4">
                  <div className="flex items-center justify-between mb-3">
                    <div className="flex items-center space-x-2">
                      <div className="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
                      <span className="text-white text-xs font-medium">Live Scanner</span>
                    </div>
                    <span className="text-blue-400 text-xs">🔍</span>
                  </div>
                  <div className="space-y-2">
                    <div className="flex justify-between text-xs">
                      <span className="text-white/70">Projects Found</span>
                      <span className="text-white font-mono">247</span>
                    </div>
                    <div className="flex justify-between text-xs">
                      <span className="text-white/70">High Probability</span>
                      <span className="text-green-400 font-mono">23</span>
                    </div>
                    <div className="w-full bg-slate-700/50 rounded-full h-1.5 mt-3">
                      <div className="bg-gradient-to-r from-blue-500 to-green-400 h-1.5 rounded-full w-3/4"></div>
                    </div>
                  </div>
                </div>

                <div className="w-56 h-32 bg-gradient-to-br from-blue-900/80 to-purple-900/80 backdrop-blur-xl rounded-2xl shadow-2xl transform -rotate-3 border border-blue-400/20 p-4 -mt-6 ml-8">
                  <div className="flex items-center space-x-2 mb-2">
                    <div className="w-2 h-2 bg-yellow-400 rounded-full animate-pulse"></div>
                    <span className="text-white text-xs font-medium">New Alert</span>
                  </div>
                  <div className="text-white text-sm font-semibold mb-1">LayerZero V2</div>
                  <div className="text-white/60 text-xs mb-2">Testnet activity detected</div>
                  <div className="flex justify-between items-center">
                    <span className="text-green-400 text-xs font-mono">95% prob</span>
                    <span className="text-white/50 text-xs">2h ago</span>
                  </div>
                </div>

                <div className="w-52 h-28 bg-gradient-to-br from-purple-900/70 to-pink-900/70 backdrop-blur-xl rounded-2xl shadow-2xl transform rotate-12 border border-purple-400/20 p-3 -mt-4">
                  <div className="flex items-center space-x-2 mb-2">
                    <span className="text-white text-xs font-medium">DAO Alpha</span>
                    <div className="w-1.5 h-1.5 bg-purple-400 rounded-full"></div>
                  </div>
                  <div className="text-white/80 text-xs mb-1">Private drop shared</div>
                  <div className="flex justify-between items-center">
                    <span className="text-purple-400 text-xs">+50 tokens</span>
                    <span className="text-white/50 text-xs">🎯</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>

      <div className="relative z-10 px-8 pb-[48px]">
        <div className="max-w-[1400px] mx-auto">
          <div className="grid md:grid-cols-3 gap-0">
            <div className="px-[32px] py-[24px] border-r border-white/[0.06] first:pl-0 last:pr-0 last:border-r-0">
              <div className="flex items-start space-x-[16px]">
                <div className="w-[40px] h-[40px] bg-white/[0.06] rounded-[10px] flex items-center justify-center flex-shrink-0 mt-[2px]">
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path
                      d="M9 12L11 14L15 10"
                      stroke="white"
                      strokeWidth="1.5"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                    <path
                      d="M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z"
                      stroke="white"
                      strokeWidth="1.5"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                  </svg>
                </div>
                <div>
                  <h3 className="text-white font-semibold text-[18px] mb-[8px] leading-[1.3] tracking-[-0.01em]">
                    Smart Detection
                  </h3>
                  <p className="text-white/45 text-[14px] leading-[1.5] font-light tracking-[0.005em]">
                    AI monitors Discord, GitHub, and
                    <br />
                    social media for early airdrop signals
                  </p>
                </div>
              </div>
            </div>

            <div className="px-[32px] py-[24px] border-r border-white/[0.06] first:pl-0 last:pr-0 last:border-r-0">
              <div className="flex items-start space-x-[16px]">
                <div className="w-[40px] h-[40px] bg-white/[0.06] rounded-[10px] flex items-center justify-center flex-shrink-0 mt-[2px]">
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path
                      d="M3 3V21L9 15L15 21L21 15V3"
                      stroke="white"
                      strokeWidth="1.5"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                    <path d="M9 9L15 3" stroke="white" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
                  </svg>
                </div>
                <div>
                  <h3 className="text-white font-semibold text-[18px] mb-[8px] leading-[1.3] tracking-[-0.01em]">
                    Probability Ranking
                  </h3>
                  <p className="text-white/45 text-[14px] leading-[1.5] font-light tracking-[0.005em]">
                    Projects ranked by airdrop probability,
                    <br />
                    deadlines, and action requirements
                  </p>
                </div>
              </div>
            </div>

            <div className="px-[32px] py-[24px] border-r border-white/[0.06] first:pl-0 last:pr-0 last:border-r-0">
              <div className="flex items-start space-x-[16px]">
                <div className="w-[40px] h-[40px] bg-white/[0.06] rounded-[10px] flex items-center justify-center flex-shrink-0 mt-[2px]">
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path
                      d="M17 21V19C17 17.9391 16.5786 16.9217 15.8284 16.1716C15.0783 15.4214 14.0609 15 13 15H5C3.93913 15 2.92172 15.4214 2.17157 16.1716C1.42143 16.9217 1 17.9391 1 19V21"
                      stroke="white"
                      strokeWidth="1.5"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                    <circle
                      cx="9"
                      cy="7"
                      r="4"
                      stroke="white"
                      strokeWidth="1.5"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                    <path
                      d="M23 21V19C23 18.1645 22.7155 17.3541 22.2094 16.7006C21.7033 16.047 20.9999 15.5866 20.2 15.3954"
                      stroke="white"
                      strokeWidth="1.5"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                    <path
                      d="M16 3.13C16.8003 3.32127 17.5037 3.78167 18.0098 4.43524C18.5159 5.08882 18.8004 5.89925 18.8004 6.735C18.8004 7.57075 18.5159 8.38118 18.0098 9.03476C17.5037 9.68833 16.8003 10.1487 16 10.34"
                      stroke="white"
                      strokeWidth="1.5"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                  </svg>
                </div>
                <div>
                  <h3 className="text-white font-semibold text-[18px] mb-[8px] leading-[1.3] tracking-[-0.01em]">
                    DAO Alpha Sharing
                  </h3>
                  <p className="text-white/45 text-[14px] leading-[1.5] font-light tracking-[0.005em]">
                    Community shares hidden alpha and
                    <br />
                    private drops for tokens and access
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
