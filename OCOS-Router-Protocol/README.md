# OCOS Router Protocol (ORP)

**OCOS Router Protocol (ORP)** — an open-source, modular, audit-ready protocol that unifies **intra-chain** (same chain) and **cross-chain** (inter-chain) routing across all blockchain platforms (EVM, TRON, Solana, Sui, Aptos, TON, NEAR, and more).

---

## 📌 Mission

In the blockchain ecosystem, achieving **fully integrated, secure, and flexible asset transfers** between chains is still complex. ORP solves this by:

- **Single standard**: unified `ORPMessage` format for all operations.
- **Adapter architecture**: multiple DEX and Bridge providers in one core protocol.
- **Audit-friendly design**: security, replay protection, timelock, and event-based tracking.

---

## 🚀 Key Features

### 🔹 **Adapter Architecture**
- **DEX Adapters** (Uniswap V2/V3, PancakeSwap, TraderJoe, etc.)
- **Bridge Adapters** (LayerZero, Axelar, Chainlink CCIP, Hyperlane, etc.)
- **Non-EVM Adapters** (Solana, Sui, Aptos, TON, Near)

### 🔹 **Unified Message Format**
```solidity
struct ORPMessage {
    uint256 version;
    uint256 srcChainId;
    uint256 dstChainId;
    bytes32 routeId;
    address srcSender;
    bytes   receiver;
    bytes   token;
    uint256 amountIn;
    uint256 minAmountOut;
    uint256 deadline;
    uint256 nonce;
    bytes   callData;
    bytes   aux;
}
```
- Replay protection: `nonce + deadline + domain separation`
- Slippage protection: `minAmountOut` + `deadline`
- Receipts & Events for full traceability

### 🔹 **Security**
- **Access Control**: `guardian`, `feeManager`, adapter allowlist
- **Pause/Unpause mechanism**
- **Rate Limiter** and token/bridge whitelists
- **Upgradeable** (UUPS or Transparent Proxy options)

---

## 📂 Project Structure

```
contracts/                 # Solidity core + adapter interfaces
  RouterCore.sol
  interfaces/
    IERC20.sol
    IDEXAdapter.sol
    IBridgeAdapter.sol
  adapters/
    CCIPAdapter.sol
script/                     # Foundry scripts
  Deploy.s.sol
test/                       # Foundry tests
packages/sdk/               # TypeScript SDK
solana/                     # Solana program skeleton
.github/                    # PR/Issue templates
```

---

## 🛠️ Installation & Usage

### With Foundry
```bash
forge build
forge test -vv
```

**Example deployment:**
```bash
forge script script/Deploy.s.sol --rpc-url $RPC --private-key $PK --broadcast
```

---

### With TypeScript SDK
```bash
cd packages/sdk
npm install
npm run build
```

**Usage:**
```typescript
import { buildMsg } from "@ocos/orp-sdk";

const { encoded, msgId } = buildMsg({
  version: 1,
  srcChainId: 56n,
  dstChainId: 1n,
  routeId: "0x" + "00".repeat(32) as `0x${string}`,
  srcSender: "0x1111111111111111111111111111111111111111",
  receiver: "0x2222222222222222222222222222222222222222",
  token: "0x3333333333333333333333333333333333333333",
  amountIn: 1000000000000000000n,
  minAmountOut: 990000000000000000n,
  deadline: BigInt(Math.floor(Date.now()/1000) + 1800),
  nonce: 1n,
  callData: "0x",
  aux: "0x"
});
```

---

## 🔗 Supported Blockchains

✅ **EVM Chains**:
- Ethereum, BNB Chain, Polygon, Arbitrum, Optimism, Avalanche, Base, zkSync, Scroll, Fantom, Gnosis Chain, and more.

✅ **Non-EVM Chains**:
- TRON (TRC-20)
- Solana
- Sui, Aptos (Move VM)
- TON, NEAR

---

## 📜 Security Guidelines

- **Mandatory audit** before mainnet usage.
- All adapters are controlled via an **allowlist**.
- Global `pause()` function allows halting the network in emergencies.

---

## 🤝 Contribution

Pull Requests and Feature Requests are welcome. For more details:
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

---

## 📄 License

MIT License — See [`LICENSE`](LICENSE) for details.

---
