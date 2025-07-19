# Governance Components Directory

**Composable | Secure | On-Chain Governance | Scalable UI**

---

## Overview

The `/governance` directory contains all React/TypeScript components for rendering, interacting with, and managing on-chain governance within the OCOS-Chain dashboard.  
These components enable users and admins to create, vote on, analyze, and execute protocol and DAO governance proposals with real-time feedback and auditability.

---

## Key Features

- **Governance-Ready:** Proposals, voting panels, result analytics, and execution controls—all in one place.
- **Composability:** Each component is atomic, reusable, and theme-aware. Combine for any governance workflow.
- **Security & Audit:** UI/UX is designed for transparency, role-based actions, and secure smart contract integration.
- **Accessibility:** Keyboard navigation, ARIA, and semantic structure by default.
- **Analytics-Enabled:** Voting history, quorum, thresholds, timeline, and analytics panels ready for DAO dashboards.
- **Enterprise-Grade:** TypeScript-typed, tested, and easy to integrate or extend.

---

## Directory Structure

```
governance/
├── GovernanceProposalList.tsx
├── GovernanceProposalCard.tsx
├── GovernanceProposalDetails.tsx
├── GovernanceVotePanel.tsx
├── GovernanceVoteResult.tsx
├── GovernanceExecutionPanel.tsx
├── GovernanceAnalyticsPanel.tsx
├── GovernanceTimeline.tsx
├── GovernanceEmptyState.tsx
├── GovernanceActionModal.tsx
├── index.tsx
```

---

## Usage Example

```tsx
import { GovernanceProposalList, GovernanceVotePanel } from "@/components/governance";

<GovernanceProposalList proposals={governanceProposals} onSelect={...} />
<GovernanceVotePanel proposal={selectedProposal} onVote={...} />
```

---

## Best Practices

- **Strong Typing:** All components and props are TypeScript-typed for maintainability.
- **UI/UX Consistency:** Built with Tailwind CSS and follows OCOS-Chain dashboard theme tokens.
- **Extensible:** Add more analytics, DAO-level custom voting, or bridge governance with minimal effort.
- **Accessible:** All inputs, tables, and panels are accessible and support keyboard/focus/ARIA.
- **Smart Contract Integration:** Designed to be fed by hooks/services reading on-chain governance state.

---

## Extending

- Add new analytics widgets, timeline components, or proposal types.
- Connect with wallet context and notification center for real-time feedback.
- Update `index.tsx` to barrel-export all new governance components.

---

## Authors & License

- **OCOS Foundation**
- [LICENSE](../../../../LICENSE)
- Contributions welcome on [GitHub](https://github.com/ocosio/dashboard)

---

*For a full component storybook or live governance demo, see the main OCOS-Chain dashboard repository.*
