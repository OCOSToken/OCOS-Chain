export const env = {
  apiBase: process.env.NEXT_PUBLIC_API_BASE ?? "",
  graphqlEndpoint: process.env.NEXT_PUBLIC_GQL_ENDPOINT ?? "",
  wsEndpoint: process.env.NEXT_PUBLIC_WS_ENDPOINT ?? "",
  appEnv: process.env.NODE_ENV ?? "development",
  sentryDsn: process.env.NEXT_PUBLIC_SENTRY_DSN ?? "",
  // Add more as needed, NEVER expose secrets or private keys
};
