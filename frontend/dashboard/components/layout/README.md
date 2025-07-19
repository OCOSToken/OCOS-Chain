# Layout Components Directory

**Universal | Modular | Accessible | Responsive UI**

---

## Overview

The `/layout` directory contains all atomic and composite React/TypeScript components for structuring the OCOS-Chain dashboard’s main user interface.  
These components deliver a unified experience across every page—enabling seamless navigation, branding, notifications, theming, and adaptive layout on any device.

---

## Key Features

- **Composable Structure:** Sidebar, navbar, mobile drawer, footer, notification center, and layout wrappers—plug and play in any dashboard page.
- **Responsive & Mobile-First:** All components scale to any device and provide mobile-specific navigation (drawer, hamburger, etc).
- **Theme & Branding Ready:** Built with Tailwind CSS, supports dark/light mode and custom branding.
- **Accessibility:** Focus management, keyboard navigation, ARIA roles, and semantic HTML for every layout part.
- **Scalable:** Easily add analytics bars, breadcrumbs, loading overlays, page containers, or global modals.
- **Enterprise-Grade:** TypeScript-typed, testable, and production UI/UX standards.

---

## Directory Structure

```
layout/
├── AppLayout.tsx
├── Sidebar.tsx
├── SidebarSection.tsx
├── Navbar.tsx
├── Header.tsx
├── Footer.tsx
├── MobileDrawer.tsx
├── Breadcrumbs.tsx
├── MainContent.tsx
├── PageContainer.tsx
├── LoadingOverlay.tsx
├── NotificationCenter.tsx
├── ThemeToggle.tsx
├── LanguageSwitcher.tsx
├── index.tsx
```

---

## Usage Example

```tsx
import { AppLayout, Sidebar, Navbar } from "@/components/layout";

<AppLayout>
  <DashboardPage />
</AppLayout>
```

---

## Best Practices

- **Do not duplicate navigation or header logic**—always use centralized layout components.
- **Update `index.tsx`** with all new/renamed components for simple barrel imports.
- **Use `ThemeToggle` and `NotificationCenter`** for global user experience features.
- **Keep mobile and desktop navigation unified through shared route configs.
- **Add `ErrorBoundary` or loading overlays for robust user flows.

---

## Extending

- **Analytics Bar:** Add a chain/network/DAO analytics component at the top or bottom.
- **Settings Panel:** Add global settings, language switcher, or user profile context.
- **Progress Indicators:** Add loading overlays or progress bars for async dashboard actions.

---

## Authors & License

- **OCOS Foundation**  
- [LICENSE](../../../../LICENSE)  
- Contribute or fork on [GitHub](https://github.com/ocosio/dashboard)

---

*For live UI demos and design tokens, see the main OCOS-Chain dashboard repository.*
