// Professional GraphQL API service for OCOS-Chain dashboard

export interface GqlOptions {
  variables?: Record<string, any>;
  headers?: Record<string, string>;
  token?: string;
  timeoutMs?: number;
  endpoint?: string; // Optionally override endpoint
}

export interface GqlError {
  message: string;
  locations?: { line: number; column: number }[];
  path?: (string | number)[];
  extensions?: any;
}

export interface GqlResponse<T> {
  data?: T;
  errors?: GqlError[];
  [key: string]: any;
}

/**
 * graphqlFetch - Universal GraphQL query/mutation fetcher with error/timeout/auth.
 * @param query - GraphQL query or mutation string
 * @param options - Variables, headers, token, timeout, endpoint
 * @returns {Promise<T>}
 */
export async function graphqlFetch<T = any>(
  query: string,
  options: GqlOptions = {}
): Promise<T> {
  const {
    variables = {},
    headers = {},
    token,
    timeoutMs = 15000,
    endpoint = "/api/graphql", // Default to your app's GQL endpoint
  } = options;

  const controller = new AbortController();
  const id = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const res = await fetch(endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        ...(token ? { Authorization: `Bearer ${token}` } : {}),
        ...headers,
      },
      body: JSON.stringify({ query, variables }),
      signal: controller.signal,
    });
    clearTimeout(id);

    const json: GqlResponse<T> = await res.json();

    if (!res.ok || json.errors) {
      const error = (json.errors && json.errors[0]) || { message: res.statusText || "GraphQL Error" };
      throw error;
    }
    return json.data as T;
  } catch (err: any) {
    if (err.name === "AbortError") {
      throw { message: "Request timeout (GraphQL)", extensions: { code: "TIMEOUT" } } as GqlError;
    }
    throw err;
  }
}

// Example helpers for query/mutation pattern
export const gqlApi = {
  // Generic query
  async query<T>(query: string, variables?: Record<string, any>, token?: string) {
    return graphqlFetch<T>(query, { variables, token });
  },

  // Generic mutation
  async mutate<T>(mutation: string, variables?: Record<string, any>, token?: string) {
    return graphqlFetch<T>(mutation, { variables, token });
  },
};
