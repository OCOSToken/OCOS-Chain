# Liquidity Components Directory

**DeFi-Ready | Modular | TVL & Yield | Web3 Composable**

---

## Overview

The `/liquidity` directory contains all reusable React/TypeScript components for displaying, managing, and interacting with liquidity pools, AMMs, swaps, farming, and rewards on the OCOS-Chain dashboard.  
These components provide atomic, scalable, and composable UI for modern DeFi experiences: real-time TVL, APY, swaps, user positions, bridge, rewards, and analytics.

---

## Key Features

- **Pool-centric UI:** Ready-to-use lists, cards, and panels for every liquidity pool.
- **AMM/DEX Operations:** Swap forms, slippage controls, price charts, and real-time on-chain data.
- **Yield Farming & Rewards:** Farm panel, pending/claimed rewards, APY, and incentives.
- **User-Centric:** User positions table, deposit/withdraw forms, historical analytics.
- **Composable & Themed:** All components are theme-aware, mobile-friendly, and ready for Tailwind/OCOS design tokens.
- **Performance:** TypeScript-typed, optimized for large TVL and pool lists, supports virtual scroll/pagination.

---

## Directory Structure

```
liquidity/
├── LiquidityPoolsList.tsx
├── LiquidityPoolCard.tsx
├── LiquidityPoolDetails.tsx
├── PoolDepositForm.tsx
├── PoolWithdrawForm.tsx
├── PoolSwapForm.tsx
├── PoolStatsPanel.tsx
├── PoolPositionsTable.tsx
├── PoolRewardsPanel.tsx
├── PoolActivityFeed.tsx
├── PoolChart.tsx
├── PoolTokensBadge.tsx
├── PoolStatusTag.tsx
├── FeeInfo.tsx
├── OraclePricePanel.tsx
├── BridgePanel.tsx
├── GovernancePanel.tsx
├── index.tsx
```

---

## Usage Example

```tsx
import { LiquidityPoolsList, PoolSwapForm } from "@/components/liquidity";

<LiquidityPoolsList pools={pools} onPoolSelect={...} />
<PoolSwapForm pool={selectedPool} onSwap={...} />
```

---

## Best Practices

- **Atomic:** Each component is responsible for one piece of the DeFi UI.
- **Data-driven:** Accepts props and hooks from on-chain or API data sources.
- **UI/UX Consistency:** Follows OCOS/Tailwind tokens and layout.
- **Testable & Extensible:** All components support unit/integration testing and are easy to extend for new pool types.
- **Mobile & Accessibility:** Full keyboard/focus, ARIA, and mobile grid layout.

---

## Extending

- **Add new chart, analytics, or farming widgets** using the same atomic component structure.
- **Integrate with hooks/services** for live TVL, APY, and trade data.
- **Update `index.tsx`** to export new liquidity modules as they are added.

---

## Authors & License

- **OCOS Foundation**  
- [LICENSE](../../../../LICENSE)  
- Contribute or fork on [GitHub](https://github.com/ocosio/dashboard)

---

*For live pool stats, design tokens, and UI demos, see the main OCOS-Chain dashboard repository.*
