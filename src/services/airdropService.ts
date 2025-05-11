export const mockAirdrops = [
    {
      name: "Airdrop 1",
      platform: "Solana",
      probability: 0.8,
      deadline: "2025-06-01",
    },
    {
      name: "Airdrop 2",
      platform: "Ethereum",
      probability: 0.6,
      deadline: "2025-07-01",
    },
  ];
  
  export const getAirdrops = async () => {
    // Мок-сервіс для отримання аірдропів
    return mockAirdrops;
  };
  