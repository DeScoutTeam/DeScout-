import React from "react";

type Props = {
  chains: string[];
  selected: string;
  onSelect: (chain: string) => void;
};

const Filters: React.FC<Props> = ({ chains, selected, onSelect }) => (
  <div className="flex space-x-2 mb-4">
    {chains.map((chain) => (
      <button
        key={chain}
        onClick={() => onSelect(chain)}
        className={`px-3 py-1 rounded-full border ${
          selected === chain
            ? "bg-purple-600 text-white"
            : "border-gray-600 text-gray-400 hover:border-purple-500 hover:text-white"
        }`}
      >
        {chain}
      </button>
    ))}
  </div>
);

export default Filters;
