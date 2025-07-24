# OCOS Dashboard Frontend

**Enterprise-Grade | Modular | Scalable | Web3-Ready**

---

## Overview

The `dashboard` directory contains the entire frontend codebase for the OCOS-Chain professional dashboard.  
It is a **modular, fully-typed, React/Next.js application** built for DeFi, DAO, wallet, analytics, and multi-chain management—designed to be scalable, maintainable, and production-ready.

---

## Key Features

- **Feature-Based Architecture:** Each domain (DAO, wallet, liquidity, governance, etc.) is encapsulated for easy scaling and development.
- **TypeScript Everywhere:** All components, hooks, context, and configs are strongly typed for IDE safety and refactorability.
- **Design System:** Uses Tailwind CSS, custom design tokens, and atomic CSS modules for consistent UI/UX and theming.
- **i18n Localization:** Ready for global, multi-language support with JSON translation files for all major world languages.
- **Web3-Enabled:** Supports EVM wallet connections, multi-chain, live data via REST, GraphQL, and WebSocket services.
- **SSR/SSG/CSR Support:** Works with Next.js for static, dynamic, or hybrid rendering.
- **Extensible:** Easily add new features, pages, or blockchain integrations.

---

## Directory Structure

```
dashboard/
├── components/    # UI building blocks (shared, layout, domain-specific)
├── pages/         # Next.js route-based pages (dashboard, DAO, pools, etc.)
├── hooks/         # Custom React hooks (wallet, proposals, etc.)
├── context/       # React context providers (wallet, theme, notification, etc.)
├── services/      # API, GraphQL, WebSocket, and blockchain integrations
├── styles/        # Global, theme, and modular CSS/Tailwind styles
├── config/        # Centralized runtime config, routes, endpoints, tokens, env
├── locales/       # All translations and i18n resource files
├── public/        # Static assets (images, fonts, icons)
├── README.md
```

---

## Getting Started

1. **Install dependencies:**
   ```bash
   npm install
   # or
   yarn
   ```

2. **Configure environment variables:**  
   Copy `.env.example` to `.env.local` and update API, GraphQL, WS endpoints, etc.

3. **Run in development mode:**
   ```bash
   npm run dev
   # or
   yarn dev
   ```

4. **Build for production:**
   ```bash
   npm run build && npm start
   # or
   yarn build && yarn start
   ```

---

## Best Practices

- **Modular code:** Add new features/components only in their domain folders.
- **Typed everywhere:** Never use untyped/any; leverage TypeScript throughout.
- **DRY & reusable:** Use hooks, context, and configs for global logic.
- **Accessible and theme-aware:** All UI follows a11y and OCOS design tokens.
- **i18n:** Keep translations in sync across all language folders in `locales/`.

---

## Extending

- Add new blockchains, wallet providers, or data modules by editing `config/`, `services/`, and `components/`.
- Add new translations by creating a new language folder in `locales/` and updating `languages.config.ts`.
- Customize theming via `styles/theme.tokens.css` and `tailwind.config.js`.

---

## Authors & License

- **OCOS Foundation**
- [LICENSE](../LICENSE)
- Contributions welcome: open issues and PRs on [GitHub](https://github.com/ocosio/dashboard)

---

*For documentation, Storybook, and live demo URLs, see the main OCOS-Chain repository.
