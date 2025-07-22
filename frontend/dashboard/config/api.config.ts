export interface ChainInfo {
  id: number;
  name: string;
  rpcUrl: string;
  explorer: string;
  currency: string;
  icon?: string; // Optional: path to network icon
  isTestnet?: boolean;
}

export const supportedChains: ChainInfo[] = [
  {
    id: 1,
    name: "Ethereum Mainnet",
    rpcUrl: "https://mainnet.infura.io/v3/YOUR_INFURA_KEY",
    explorer: "https://etherscan.io",
    currency: "ETH",
    icon: "/networks/eth.svg",
  },
  {
    id: 56,
    name: "BNB Chain",
    rpcUrl: "https://bsc-dataseed.binance.org",
    explorer: "https://bscscan.com",
    currency: "BNB",
    icon: "/networks/bnb.svg",
  },
  {
    id: 137,
    name: "Polygon",
    rpcUrl: "https://polygon-rpc.com",
    explorer: "https://polygonscan.com",
    currency: "MATIC",
    icon: "/networks/polygon.svg",
  },
  {
    id: 10,
    name: "Optimism",
    rpcUrl: "https://mainnet.optimism.io",
    explorer: "https://optimistic.etherscan.io",
    currency: "ETH",
    icon: "/networks/optimism.svg",
  },
  {
    id: 42161,
    name: "Arbitrum One",
    rpcUrl: "https://arb1.arbitrum.io/rpc",
    explorer: "https://arbiscan.io",
    currency: "ETH",
    icon: "/networks/arbitrum.svg",
  },
  {
    id: 250,
    name: "Fantom",
    rpcUrl: "https://rpc.ftm.tools",
    explorer: "https://ftmscan.com",
    currency: "FTM",
    icon: "/networks/fantom.svg",
  },
  {
    id: 43114,
    name: "Avalanche C-Chain",
    rpcUrl: "https://api.avax.network/ext/bc/C/rpc",
    explorer: "https://snowtrace.io",
    currency: "AVAX",
    icon: "/networks/avalanche.svg",
  },
  {
    id: 100,
    name: "Gnosis Chain (xDai)",
    rpcUrl: "https://rpc.gnosischain.com",
    explorer: "https://gnosisscan.io",
    currency: "xDAI",
    icon: "/networks/gnosis.svg",
  },
  {
    id: 8217,
    name: "Klaytn",
    rpcUrl: "https://public-node-api.klaytnapi.com/v1/cypress",
    explorer: "https://scope.klaytn.com",
    currency: "KLAY",
    icon: "/networks/klaytn.svg",
  },
  {
    id: 1284,
    name: "Moonbeam",
    rpcUrl: "https://rpc.api.moonbeam.network",
    explorer: "https://moonscan.io",
    currency: "GLMR",
    icon: "/networks/moonbeam.svg",
  },
  {
    id: 2222,
    name: "Kava EVM",
    rpcUrl: "https://evm.kava.io",
    explorer: "https://explorer.kava.io",
    currency: "KAVA",
    icon: "/networks/kava.svg",
  },
  {
    id: 25,
    name: "Cronos",
    rpcUrl: "https://evm.cronos.org",
    explorer: "https://cronoscan.com",
    currency: "CRO",
    icon: "/networks/cronos.svg",
  },
  // Testnets (for dev purposes)
  {
    id: 5,
    name: "Goerli (Ethereum Testnet)",
    rpcUrl: "https://goerli.infura.io/v3/YOUR_INFURA_KEY",
    explorer: "https://goerli.etherscan.io",
    currency: "ETH",
    icon: "/networks/eth.svg",
    isTestnet: true,
  },
  {
    id: 97,
    name: "BNB Testnet",
    rpcUrl: "https://data-seed-prebsc-1-s1.binance.org:8545",
    explorer: "https://testnet.bscscan.com",
    currency: "tBNB",
    icon: "/networks/bnb.svg",
    isTestnet: true,
  },
  {
    id: 80001,
    name: "Polygon Mumbai",
    rpcUrl: "https://rpc-mumbai.maticvigil.com",
    explorer: "https://mumbai.polygonscan.com",
    currency: "MATIC",
    icon: "/networks/polygon.svg",
    isTestnet: true,
  },
  {
    id: 1337,
    name: "Localhost (Hardhat/EVM)",
    rpcUrl: "http://127.0.0.1:8545",
    explorer: "",
    currency: "ETH",
    isTestnet: true,
  }
];

export const apiConfig = {
  restBaseUrl: process.env.NEXT_PUBLIC_API_BASE || "https://api.ocos.io",
  graphqlEndpoint: process.env.NEXT_PUBLIC_GQL_ENDPOINT || "https://api.ocos.io/graphql",
  wsEndpoint: process.env.NEXT_PUBLIC_WS_ENDPOINT || "wss://ws.ocos.io",
  chainId: 56, // Default chain (BNB Chain mainnet)
  explorerBaseUrl: "https://explorer.ocos.io",
  supportedChains,
  chainName: "OCOS Chain"
};
