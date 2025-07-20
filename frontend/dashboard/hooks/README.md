# Custom React Hooks Directory

**Composable | Typed | Modular | Best Practices**

---

## Overview

The `/hooks` directory contains all custom React hooks for the OCOS-Chain dashboard.  
Hooks here encapsulate **reusable business logic**, state management, API data fetching, blockchain integrations, UI utilities, and more—enabling DRY, clean, and testable code across the whole frontend.

---

## Key Features

- **Typed & Modular:** Each hook is written in TypeScript and encapsulates one focused concern.
- **Composable:** Designed for maximum reuse in pages, components, or providers.
- **Best Practices:** Hooks abstract logic away from UI, supporting clean separation of concerns.
- **SSR & CSR Ready:** All hooks are compatible with Next.js SSR/ISR and client-side data fetching.
- **Web3 & DeFi-Ready:** Support for wallet, DAO, liquidity, governance, and blockchain state.
- **Testable:** Easy to write unit and integration tests for hooks in isolation.

---

## Directory Structure

```
hooks/
├── useWallet.ts             # Wallet connect, status, network/session logic
├── useDaoProposals.ts       # Fetch/filter DAO proposals (API, GraphQL, or on-chain)
├── useLiquidityPools.ts     # List and manage DeFi pools, filters, pagination
├── useTokenPrices.ts        # Fetch token prices from oracle or price APIs
├── useUserTransactions.ts   # Fetch user/DAO transaction history
├── useGovernance.ts         # Voting, delegation, governance proposal state
├── useNotifications.ts      # App-wide toast/alert state management
├── useDarkMode.ts           # Dark/light mode toggle and persistence
├── useDebounce.ts           # Debounce any value (search/filter performance)
├── index.ts                 # Barrel export for all hooks
```

---

## Usage Example

```tsx
import { useWallet, useDaoProposals, useDarkMode } from "@/hooks";

const { address, connect } = useWallet();
const { proposals, loading } = useDaoProposals({ status: "Pending" });
const { isDark, toggle } = useDarkMode();
```

---

## Best Practices

- **One hook per concern:** Each hook should do one thing and do it well.
- **Typed return values:** Always provide full types for data, loading, error, and control APIs.
- **Keep hooks logic-only:** Do not return UI; hooks are for logic, state, and effects.
- **Composable:** Build complex state/logic by composing multiple hooks together.
- **Update `index.ts`** when adding or removing hooks.

---

## Extending

- Add hooks for modals, forms, data caching, analytics, WebSocket, subscriptions, etc.
- Connect hooks to providers or state containers as your app scales.
- Add tests in a `/__tests__/` or `/__mocks__/` folder for each hook.

---

## Authors & License

- **OCOS Foundation**
- [LICENSE](../../../LICENSE)
- For contributions, raise pull requests on [GitHub](https://github.com/ocosio/dashboard)

---

*For advanced hook usage, SSR/API patterns, and integration examples, see the main OCOS-Chain dashboard repository.
