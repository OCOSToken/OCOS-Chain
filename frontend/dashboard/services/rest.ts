// Professional REST API service for OCOS-Chain dashboard

export interface RestOptions {
  method?: "GET" | "POST" | "PUT" | "DELETE";
  headers?: Record<string, string>;
  body?: any;
  token?: string;                // Optional: For auth/session
  timeoutMs?: number;            // Optional: Request timeout in ms
}

export interface RestError {
  status: number;
  message: string;
  details?: any;
}

/**
 * restFetch - Universal REST API fetcher with error and timeout handling.
 * @param endpoint - Relative API endpoint (e.g. /api/pools)
 * @param options  - Method, headers, body, etc.
 */
export async function restFetch<T = any>(
  endpoint: string,
  options: RestOptions = {}
): Promise<T> {
  const {
    method = "GET",
    headers = {},
    body,
    token,
    timeoutMs = 15000,
  } = options;

  const controller = new AbortController();
  const id = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const res = await fetch(endpoint, {
      method,
      headers: {
        "Content-Type": "application/json",
        ...(token ? { Authorization: `Bearer ${token}` } : {}),
        ...headers,
      },
      body: body ? JSON.stringify(body) : undefined,
      signal: controller.signal,
    });
    clearTimeout(id);

    const contentType = res.headers.get("Content-Type");
    const data = contentType && contentType.includes("application/json")
      ? await res.json()
      : await res.text();

    if (!res.ok) {
      throw {
        status: res.status,
        message: data?.message || res.statusText || "API Error",
        details: data,
      } as RestError;
    }

    return data as T;
  } catch (err: any) {
    if (err.name === "AbortError") {
      throw { status: 408, message: "Request timeout", details: null } as RestError;
    }
    throw err;
  }
}

// Example helpers for resource-centric calls
export const restApi = {
  // GET resource
  async get<T>(endpoint: string, token?: string) {
    return restFetch<T>(endpoint, { method: "GET", token });
  },

  // POST resource
  async post<T>(endpoint: string, body?: any, token?: string) {
    return restFetch<T>(endpoint, { method: "POST", body, token });
  },

  // PUT resource
  async put<T>(endpoint: string, body?: any, token?: string) {
    return restFetch<T>(endpoint, { method: "PUT", body, token });
  },

  // DELETE resource
  async del<T>(endpoint: string, token?: string) {
    return restFetch<T>(endpoint, { method: "DELETE", token });
  },
};
