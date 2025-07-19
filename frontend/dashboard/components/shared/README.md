# Shared UI Components Directory

**Atomic | Universal | Reusable | Theme-Ready**

---

## Overview

The `/shared` directory contains all atomic, universal, and reusable UI components used across the OCOS-Chain dashboard.  
These foundational elements power forms, modals, feedback, notifications, inputs, and more—enabling UI consistency, rapid development, and world-class UX.

---

## Key Features

- **Atomic & Composable:** Each component is focused, stateless, and easily composed into higher-level modules.
- **Accessibility by Default:** Focus rings, ARIA labels, keyboard navigation, and semantic HTML.
- **Theme & Brand-Ready:** Fully Tailwind-compatible, works with dark/light modes and custom tokens.
- **Production UX:** Loading, error, success, and feedback states are covered for every common UI pattern.
- **Typed & Tested:** Built with TypeScript and easily unit/integration tested.
- **Performance:** Tree-shakable, highly optimized for reusability.

---

## Directory Structure

```
shared/
├── Button.tsx
├── Input.tsx
├── Select.tsx
├── Checkbox.tsx
├── Radio.tsx
├── Switch.tsx
├── TextArea.tsx
├── Badge.tsx
├── Tooltip.tsx
├── Spinner.tsx
├── Modal.tsx
├── Drawer.tsx
├── Toaster.tsx
├── ProgressBar.tsx
├── ErrorBoundary.tsx
├── Avatar.tsx
├── Pagination.tsx
├── Skeleton.tsx
├── Upload.tsx
├── CopyToClipboard.tsx
├── index.tsx
```

---

## Usage Example

```tsx
import { Button, Input, Modal, Toaster } from "@/components/shared";

<Button variant="primary" loading>Save Changes</Button>
<Input label="Username" placeholder="Enter your username" />
<Modal isOpen={open} title="Confirm" onClose={closeModal}>Are you sure?</Modal>
<Toaster />
```

---

## Best Practices

- **No business logic:** These are presentational components only; keep data fetching and state in hooks or parent containers.
- **Consistent props:** All components follow predictable naming for `label`, `error`, `size`, `variant`, etc.
- **Typed everywhere:** TypeScript is used for every component and prop for reliability.
- **Composable:** Mix and match in any UI: forms, dashboards, modals, tables, and more.

---

## Extending

- Add new atoms (icons, chips, loaders) or molecules (custom form controls).
- Update `index.tsx` for easy barrel-imports throughout your codebase.
- Write stories and unit tests for every component.

---

## Authors & License

- **OCOS Foundation**  
- [LICENSE](../../../../LICENSE)  
- Contribute or fork on [GitHub](https://github.com/ocosio/dashboard)

---

*For UI tokens, global styles, and design references, see the main OCOS-Chain dashboard repository.*
