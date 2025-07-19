# Wallet Components Directory

**Web3 Ready | Multi-Chain | Secure | Modular**

---

## Overview

The `/wallet` directory contains all composable React/TypeScript components for wallet connection, account management, asset tracking, and transaction workflows in the OCOS-Chain dashboard.  
These components provide the bridge between end-users and the OCOS/DeFi protocol, enabling a seamless, secure, and feature-rich wallet experience across multiple chains and assets.

---

## Key Features

- **Universal Wallet UX:** Supports MetaMask, WalletConnect, browser wallets, hardware wallets, and custom integrations.
- **Multi-Chain Support:** Ready for Ethereum, BNB, Polygon, OCOS-Chain, and any EVM-compatible networks.
- **Security by Design:** Connection status, session info, copy warnings, disconnect controls, and security panel.
- **User-Centric:** QR code receive, send funds, NFT gallery, transaction history, and native + token balances.
- **Composable:** Each component is standalone, easily integrated into navbars, dashboards, or modals.
- **Production Quality:** Typed, tested, and theme-aware for light/dark/brand support.

---

## Directory Structure

```
wallet/
├── WalletConnectButton.tsx
├── WalletStatus.tsx
├── WalletAccountMenu.tsx
├── WalletBalancePanel.tsx
├── WalletTxHistory.tsx
├── WalletReceiveModal.tsx
├── WalletSendModal.tsx
├── WalletNetworkSelector.tsx
├── WalletSecurityPanel.tsx
├── WalletSignaturePanel.tsx
├── WalletAssetsTable.tsx
├── WalletNftGallery.tsx
├── WalletActionModal.tsx
├── index.tsx
```

---

## Usage Example

```tsx
import { WalletConnectButton, WalletBalancePanel } from "@/components/wallet";

<WalletConnectButton
  isConnected={!!address}
  address={address}
  onConnect={openWalletModal}
  onDisconnect={disconnectWallet}
/>
<WalletBalancePanel balances={userBalances} totalUsd={walletUsdValue} />
```

---

## Best Practices

- **Presentational only:** State, data-fetching, and on-chain logic should be handled in hooks or service layers.
- **Security first:** Always provide users with clear session info, disconnect options, and copy-to-clipboard feedback.
- **Typed everywhere:** All props and returned values are TypeScript-typed for reliability.
- **Accessible:** ARIA labels, focus rings, keyboard shortcuts, and responsive design.
- **Multi-chain ready:** Use network selectors and context to enable multi-chain experiences.

---

## Extending

- **Add custom wallet provider support** (Ledger, Trezor, Safe, etc.)
- **Integrate with notifications, analytics, or token price feeds.**
- **Update `index.tsx`** to export any new wallet-related modules or widgets.

---

## Authors & License

- **OCOS Foundation**  
- [LICENSE](../../../../LICENSE)  
- Contribute or fork on [GitHub](https://github.com/ocosio/dashboard)

---

*For wallet UX demos and security guidelines, see the main OCOS-Chain dashboard repository.*
