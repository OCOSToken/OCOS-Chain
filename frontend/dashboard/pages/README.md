# Pages Directory

**SSR-Ready | SEO-Optimized | Modular | Next.js Best Practice**

---

## Overview

The `/pages` directory holds all route-based page components for the OCOS-Chain dashboard.  
It leverages Next.js's file-based routing, enabling fast, SEO-optimized, and scalable navigation across all user flows—from DAO and DeFi explorers to wallet dashboards and analytics.

---

## Key Features

- **Automatic Routing:** Each `.tsx` file becomes a URL endpoint, supporting clean navigation and deep linking.
- **SEO & SSR:** Built-in support for `<Head>`, server-side rendering, dynamic meta, and OG tags for all public-facing pages.
- **Data Fetching:** Use Next.js `getServerSideProps` or hooks (SWR/React Query) for live, real-time dashboard data.
- **Composability:** Pages are assembled from atomic components (`/components/`), keeping UI consistent and code DRY.
- **TypeScript by Default:** Every page is strongly typed for safer, more productive development.

---

## Directory Structure

```
pages/
├── index.tsx            # Dashboard Home / Landing
├── dao.tsx              # DAO explorer
├── pools.tsx            # Liquidity pools/DeFi explorer
├── governance.tsx       # Governance explorer
├── wallet.tsx           # User wallet dashboard
├── analytics.tsx        # Analytics dashboard
├── ...                  # (Add additional pages here)
```

---

## Usage Example

```tsx
// Example: /pages/dao.tsx
import { AppLayout } from "@/components/layout";
import { DaoList } from "@/components/dao";

export default function DaoExplorer() {
  return (
    <AppLayout>
      <h1>DAO Explorer</h1>
      <DaoList daos={...} />
    </AppLayout>
  );
}
```

---

## Best Practices

- **Place only page-level route components here.** All UI should be built from `/components/`.
- **Keep pages as presentational shells.** Fetch data and pass to components as props.
- **Always wrap with `<AppLayout>`.** For consistent navigation, theme, and notifications.
- **Add `<Head>` with unique title/description for every page.**
- **Export only one default React component per file.**

---

## Extending

- Add dynamic routes with `[param].tsx` for DAO, proposal, or pool detail.
- Create API endpoints in `/pages/api/` if needed for custom server logic.
- Use Next.js middleware for authentication or redirects.

---

## Authors & License

- **OCOS Foundation**
- [LICENSE](../../../LICENSE)
- For contributions, raise pull requests on [GitHub](https://github.com/ocosio/dashboard)

---

*For SSR/API usage, deployment scripts, and CI/CD, see the main OCOS-Chain monorepo.*
