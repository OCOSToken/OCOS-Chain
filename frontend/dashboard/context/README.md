# App Context Providers Directory

**Global State | Modular | Typed | Enterprise-grade**

---

## Overview

The `/context` directory contains all global React context providers for the OCOS-Chain dashboard.  
Contexts here manage **shared state, UI, and services**—such as wallet connection, theming, notifications, DAO state, language/locale, authentication, and analytics—accessible throughout your application tree.

---

## Key Features

- **Centralized Global State:** All shared, app-wide state and services are managed in one place.
- **Typed Providers:** Each context is TypeScript-typed for safety and IDE support.
- **Composability:** Contexts are modular; only include what your component or page needs.
- **Enterprise Practices:** Use for wallet, theme, notification, DAO, i18n, analytics, etc.
- **SSR/CSR Safe:** Designed to work with Next.js, SSR, and modern React.
- **Testable:** Context logic is easily unit tested and mocked.

---

## Directory Structure

```
context/
├── WalletContext.tsx        # Web3 wallet, provider, and session state
├── ThemeContext.tsx         # Dark/light mode and branding
├── DaoContext.tsx           # Current DAO, proposals, and DAO-scoped settings
├── NotificationContext.tsx  # Toast/alert and notification state
├── LanguageContext.tsx      # Locale, i18n, language switching
├── AuthContext.tsx          # User auth, session, and permissions
├── AnalyticsContext.tsx     # Analytics/tracking event dispatcher
├── index.ts                 # Barrel export for all contexts
```

---

## Usage Example

```tsx
import { WalletProvider, ThemeProvider, NotificationProvider } from "@/context";

function App({ children }) {
  return (
    <WalletProvider>
      <ThemeProvider>
        <NotificationProvider>
          {children}
        </NotificationProvider>
      </ThemeProvider>
    </WalletProvider>
  );
}

// In a component:
import { useWalletContext, useNotificationContext } from "@/context";

const { address, connect } = useWalletContext();
const { show } = useNotificationContext();
```

---

## Best Practices

- **Wrap the app** with all required context providers (in `_app.tsx`, `layout.tsx`, or `AppLayout`).
- **One context per feature area:** Avoid “god contexts”—keep each focused and independent.
- **Typed interfaces:** All context values and hooks should be fully typed.
- **Keep context logic pure:** Side effects and API calls should be managed inside provider components.
- **Encapsulate only global/shared state:** Local component state belongs in components or hooks.

---

## Extending

- Add new contexts for modals, forms, sockets, feature flags, or custom domain logic.
- Export all providers and hooks via `index.ts` for simple, clean imports.
- Use context composition to support per-page or per-feature custom state.

---

## Authors & License

- **OCOS Foundation**
- [LICENSE](../../../LICENSE)
- For contributions, raise pull requests on [GitHub](https://github.com/ocosio/dashboard)

---

*For advanced usage, SSR-safe providers, and testing patterns, see the main OCOS-Chain dashboard repository.*
