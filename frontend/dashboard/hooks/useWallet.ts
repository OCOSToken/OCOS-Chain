import { useEffect, useState, useCallback } from "react";
import { ethers } from "ethers";

interface WalletState {
  address: string | null;
  chainId: number | null;
  provider: ethers.providers.Web3Provider | null;
  isConnected: boolean;
  connect: () => Promise<void>;
  disconnect: () => void;
}

/**
 * useWallet
 * A reusable hook to manage wallet connection, state, and provider access.
 */
export function useWallet(): WalletState {
  const [address, setAddress] = useState<string | null>(null);
  const [chainId, setChainId] = useState<number | null>(null);
  const [provider, setProvider] = useState<ethers.providers.Web3Provider | null>(null);

  const connect = useCallback(async () => {
    if (typeof window === "undefined" || !window.ethereum) {
      console.warn("No wallet provider detected.");
      return;
    }

    try {
      const web3Provider = new ethers.providers.Web3Provider(window.ethereum);
      await web3Provider.send("eth_requestAccounts", []);
      const signer = web3Provider.getSigner();
      const userAddress = await signer.getAddress();
      const network = await web3Provider.getNetwork();

      setAddress(userAddress);
      setChainId(network.chainId);
      setProvider(web3Provider);

      localStorage.setItem("wallet_address", userAddress);
    } catch (err) {
      console.error("Wallet connection failed:", err);
    }
  }, []);

  const disconnect = useCallback(() => {
    setAddress(null);
    setChainId(null);
    setProvider(null);
    localStorage.removeItem("wallet_address");
  }, []);

  useEffect(() => {
    const restoreSession = async () => {
      const savedAddress = localStorage.getItem("wallet_address");
      if (savedAddress && window.ethereum) {
        const web3Provider = new ethers.providers.Web3Provider(window.ethereum);
        const network = await web3Provider.getNetwork();
        setAddress(savedAddress);
        setChainId(network.chainId);
        setProvider(web3Provider);
      }
    };
    restoreSession();
  }, []);

  return {
    address,
    chainId,
    provider,
    isConnected: !!address,
    connect,
    disconnect,
  };
}
