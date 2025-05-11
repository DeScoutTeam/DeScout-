import React from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import { WalletContextProvider } from "./context/WalletContext";
import HomePage from "./pages/HomePage";
import WalletPage from "./pages/WalletPage";
import CommunityPage from "./pages/CommunityPage";
import Navbar from "./components/Navbar";
import WalletButton from "./components/WalletButton";
import WalletInfo from "./components/WalletInfo";
import AirdropList from "./components/WalletButton";

const App: React.FC = () => (
  <Router>
    <WalletContextProvider>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/wallet" element={<WalletPage />} />
        <Route path="/community" element={<CommunityPage />} />
      </Routes>
      {/* <WalletButton /> */}
        {/* <WalletInfo /> */}
        
       
        {/* <AirdropList /> */}
    </WalletContextProvider>
  </Router>
);

export default App;
