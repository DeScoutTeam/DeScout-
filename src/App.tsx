import React from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import { WalletContextProvider } from "./context/WalletContext";
import HomePage from "./pages/HomePage";

import '@solana/wallet-adapter-react-ui/styles.css';

const App: React.FC = () => (
  <Router>
    <WalletContextProvider>
      <Routes>
        <Route path="/" element={<HomePage />} />
      </Routes>
    </WalletContextProvider>
  </Router>
);

export default App;
