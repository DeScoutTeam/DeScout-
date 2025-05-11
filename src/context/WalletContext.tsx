import React, { createContext, useContext, useMemo } from "react";
import {
  ConnectionProvider,
  useWallet,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter, // Замінили Sollet на Solflare
} from "@solana/wallet-adapter-wallets";
import { clusterApiUrl, PublicKey } from "@solana/web3.js";

// Типізація для WalletContext
interface WalletContextProps {
    connected: boolean;
    publicKey: PublicKey | null;
    connect: () => void;
    disconnect: () => void;
}
interface WalletContextProviderProps {
    children: React.ReactNode; // Додаємо тип для дітей компонента
  }



const WalletContext = createContext<WalletContextProps | undefined>(undefined);

export const useSolanaWallet = (): WalletContextProps => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error("useSolanaWallet must be used within a WalletContextProvider");
  }
  return context;
};

export const WalletContextProvider: React.FC<WalletContextProviderProps> = ({ children }) => {
  const network = "mainnet-beta"; // Можна змінювати на devnet, testnet
  const endpoint = clusterApiUrl(network);

  const wallets = useMemo(
    () => [new PhantomWalletAdapter(), new SolflareWalletAdapter()],
    []
  );
  const { connected, publicKey, connect, disconnect } = useWallet()
  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletContext.Provider value={{ connected, publicKey, connect, disconnect }}>
          {children}
        </WalletContext.Provider>
      </WalletProvider>
    </ConnectionProvider>
  );
};
