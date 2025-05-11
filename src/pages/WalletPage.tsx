import React from "react";
import WalletConnect from "../components/WalletConnect";

const WalletPage: React.FC = () => (
  <div className="p-6 text-white">
    <h2 className="text-2xl font-bold mb-4">Підключення гаманця</h2>
    <WalletConnect />
  </div>
);

export default WalletPage;
