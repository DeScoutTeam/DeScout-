import React from "react";
import { useSolanaWallet } from "../context/WalletContext"; // Ваш контекст для гаманця

const WalletInfo = () => {
  const { connected, publicKey } = useSolanaWallet();

  if (!connected) return null;

  return (
    <div className="p-4 border rounded-lg shadow-md">
      <h3 className="text-lg font-bold">Wallet Info</h3>
      <p>Public Key: {publicKey?.toBase58()}</p>
      <p>Status: {connected ? "Connected" : "Disconnected"}</p>
    </div>
  );
};

export default WalletInfo;
