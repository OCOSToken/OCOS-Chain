# Dashboard Components

**Modular | Typed | Reusable | Enterprise-grade**

---

## Overview

The `/components` directory is the centralized hub for all reusable UI elements and page-level building blocks in the OCOS-Chain dashboard.  
Each subdirectory within it follows a modular, feature-based structure, enabling efficient scaling, easy maintenance, and high developer velocity for enterprise-level Web3 apps.

---

## Architecture Philosophy

- **Atomic Design Principles**  
  Each component is designed to do one thing well—smaller atoms (`shared/`) compose molecules (`layout/`), which power organisms (`dao/`, `wallet/`, etc).

- **Fully Typed**  
  Every component and prop is written in TypeScript, ensuring type safety, IDE support, and self-documented contracts.

- **Accessible by Default**  
  All components support keyboard navigation, ARIA roles, focus outlines, and screen reader compatibility.

- **Theme-Aware**  
  Fully compatible with Tailwind CSS and OCOS branding tokens. Dark/light modes supported out-of-the-box.

- **Composable & Isolated**  
  Easily unit-tested, mocked, and rendered in isolation for previews, documentation, or Storybook.

---

## Directory Structure

```
components/
├── dao/           # Governance modules for DAOs (proposals, voting, members, etc)
├── governance/    # System-level on-chain governance UIs
├── liquidity/     # AMM pools, yield, TVL, swap, analytics
├── wallet/        # Wallet connection, balance, NFT, tx history
├── layout/        # App shell: sidebar, navbar, drawer, theme
├── shared/        # Atomic design system: buttons, modals, form inputs, toast
```

---

## Usage Example

```tsx
import { Button, Input } from "@/components/shared";
import { AppLayout } from "@/components/layout";
import { DaoProposalList } from "@/components/dao";

<AppLayout>
  <DaoProposalList proposals={daoProposals} />
</AppLayout>
```

---

## Best Practices

- **Barrel exports (`index.tsx`)** are provided in every folder for clean imports.
- **Keep components stateless** where possible—pass all logic via props or context.
- **Encapsulate styles and interactions** inside each file—no external mutation.
- **Separate presentational and container components** for clarity and testability.

---

## Extending

- Add new components in the correct domain directory (e.g., `wallet/WalletBackupPanel.tsx`).
- Always update `index.tsx` for re-exports.
- Keep one component per file unless composing internally.
- Follow Tailwind naming conventions and OCOS UI guidelines.

---

## Authors & License

- **OCOS Foundation**  
- [LICENSE](../../../LICENSE)  
- For contributions, raise pull requests on [GitHub](https://github.com/ocosio/dashboard)

---

*For storybook previews, testing utilities, and design tokens, refer to the main OCOS-Chain monorepo.*
