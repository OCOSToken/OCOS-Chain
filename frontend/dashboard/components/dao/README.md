# DAO Components Directory

**Modular | Accessible | Scalable | Web3-Ready**

---

## Overview

The `/dao` folder contains all reusable React/TypeScript UI components for visualizing, managing, and interacting with DAOs on the OCOS-Chain dashboard.  
This includes everything from proposal lists to vote panels, treasury analytics, member management, and governance action modals.

---

## Key Features

- **Atomic Design:** Each component handles a single, well-defined function—composable for any DAO page or widget.
- **Accessibility:** Keyboard navigation, ARIA roles, focus rings, and semantic tags by default.
- **Theme & Brand Ready:** All components are Tailwind/dark mode compatible, with flexible props for DAO branding (logo, color, badge).
- **Enterprise-Grade:** Fully TypeScript-typed, easily extendable, production UI/UX standards.
- **Web3 Integrations:** Supports wallet-based voting, proposal state from on-chain data, and real-time governance activity.

---

## Directory Structure

```
dao/
├── DaoOverviewCard.tsx
├── DaoList.tsx
├── DaoDetailsPanel.tsx
├── DaoProposalList.tsx
├── DaoProposalCard.tsx
├── DaoProposalDetails.tsx
├── DaoVotePanel.tsx
├── DaoVoteResult.tsx
├── DaoMembersTable.tsx
├── DaoRoleBadge.tsx
├── DaoTreasuryPanel.tsx
├── DaoActionModal.tsx
├── DaoTimeline.tsx
├── DaoEmptyState.tsx
├── DaoActivityFeed.tsx
├── DaoDocsLinks.tsx
├── DaoBadge.tsx
├── index.tsx
```

---

## Usage Example

```tsx
import { DaoProposalList, DaoMembersTable } from "@/components/dao";

<DaoProposalList proposals={proposals} onProposalSelect={...} />
<DaoMembersTable members={daoMembers} showActions={isAdmin} />
```

---

## Best Practices

- **TypeScript:** All props and data types are strongly typed for reliability.
- **Composable Props:** Accept onClick, onAction, and UI slot props for flexible composition in modals, panels, or pages.
- **Testable:** Each component is easily tested in isolation (unit/integration).
- **Style:** Uses Tailwind CSS for rapid theme customization and dark/light support.
- **Accessibility:** Designed to meet a11y standards out-of-the-box.

---

## Extending

- **Add new DAO widgets** (stats, proposal analytics, NFT-based membership, etc.) by following the atomic component pattern.
- **Integrate with hooks, services, or state/context** for live data.
- **Update `index.tsx`** to barrel-export any new components.

---

## Authors & License

- **OCOS Foundation**  
- [LICENSE](../../../../LICENSE)  
- Contribute or fork on [GitHub](https://github.com/ocosio/dashboard)

---

*For further docs, component storybook, or design tokens, see the main dashboard repo.*
