import React, { useEffect, useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletNotConnectedError } from "@solana/wallet-adapter-base";

const WalletConnect: React.FC = () => {
  const { connected, publicKey, connect, disconnect, wallet } = useWallet();
  const [address, setAddress] = useState<string | null>(null);

  useEffect(() => {
    if (publicKey) {
      setAddress(publicKey.toString());
    }
  }, [publicKey]);

  const handleConnect = async () => {
    try {
      if (wallet && !connected) {
        await connect();
      }
    } catch (error) {
      if (error instanceof WalletNotConnectedError) {
        console.error("Wallet not connected", error);
      }
    }
  };

  const handleDisconnect = async () => {
    try {
      await disconnect();
      setAddress(null);
    } catch (error) {
      console.error("Disconnect error:", error);
    }
  };

  return (
    <div>
      {connected ? (
        <div>
          <p className="text-green-400">Підключено: {address}</p>
          <button
            onClick={handleDisconnect}
            className="mt-2 bg-red-600 text-white px-4 py-2 rounded"
          >
            Відключити
          </button>
        </div>
      ) : (
        <button
          onClick={handleConnect}
          className="mt-2 bg-blue-600 text-white px-4 py-2 rounded"
        >
          Підключити гаманець
        </button>
      )}
    </div>
  );
};

export default WalletConnect;
