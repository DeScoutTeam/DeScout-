import { useEffect, useState } from "react";
import { getAirdrops } from "../services/airdropService"; // Мок-сервіс

const AirdropList = () => {
  const [airdrops, setAirdrops] = useState< Airdrop[]>([]);
  interface Airdrop {
    name: string;
    platform: string;
    probability: number;  // ймовірність (наприклад, 0.8 для 80%)
    deadline: string;  // дата дедлайну, у форматі "YYYY-MM-DD"
  }
  useEffect(() => {
    const fetchAirdrops = async () => {
      const data = await getAirdrops();
      setAirdrops(data);
    };

    fetchAirdrops();
  }, []);

  return (
    <div className="space-y-4">
      {airdrops.map((airdrop, index) => (
        <div key={index} className="p-4 border rounded-lg shadow-md">
          <h3 className="text-lg font-bold">{airdrop.name}</h3>
          <p>Platform: {airdrop.platform}</p>
          <p>Probability: {airdrop.probability * 100}%</p>
          <p>Deadline: {airdrop.deadline}</p>
        </div>
      ))}
    </div>
  );
};

export default AirdropList;
