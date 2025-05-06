import React, { useState, useEffect } from "react";
import Filters from "../components/Filters";
import ProjectCard from "../components/ProjectCard";
import { Airdrop } from "../types";

const mockAirdrops: Airdrop[] = [
  { id: 1, name: "SolanaBoost", description: "Stake to earn", chain: "Solana", deadline: "2025-05-12", action: "Stake" },
  { id: 2, name: "EthVault", description: "Invite friends", chain: "Ethereum", deadline: "2025-06-01", action: "Invite" },
  { id: 3, name: "TestDrop", description: "Testnet actions", chain: "Polygon", deadline: "2025-06-15", action: "Tasks" },
];

const HomePage: React.FC = () => {
  const [airdrops, setAirdrops] = useState<Airdrop[]>([]);
  const [selectedChain, setSelectedChain] = useState<string>("All");

  useEffect(() => {
    setAirdrops(mockAirdrops);
  }, []);

  const filtered = selectedChain === "All"
    ? airdrops
    : airdrops.filter((a) => a.chain === selectedChain);

  return (
    <div className="p-6">
      <Filters
        chains={["All", "Solana", "Ethereum", "Polygon"]}
        selected={selectedChain}
        onSelect={setSelectedChain}
      />
      <div className="space-y-4">
        {filtered.map((drop) => (
          <ProjectCard key={drop.id} project={drop} />
        ))}
      </div>
    </div>
  );
};

export default HomePage;
