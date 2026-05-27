import CONFIG from "@/lib/config";

export interface ApiError {
  message: string;
  status: number;
}

export type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export interface FetchOptions {
  method?: HttpMethod;
  body?: unknown;
  headers?: Record<string, string>;
}

export async function handleResponse<T>(res: Response): Promise<T> {
  if (res.status === 401) {
    // Token is invalid or expired — clear it so the user is prompted to log in again
    localStorage.removeItem(CONFIG.tokenKey);
  }
  if (!res.ok) {
    let message = res.statusText;
    try {
      const body = await res.text();
      if (body) message = body;
    } catch {
      // ignore parse error
    }
    throw { message, status: res.status } satisfies ApiError;
  }
  return res.json() as Promise<T>;
}

/**
 * Authenticated fetch wrapper. Automatically attaches the stored Bearer token
 * to every request. Developers only need to supply the path, method, and body.
 *
 * @example
 * const data = await apiFetch<Device[]>("/devices");
 * const created = await apiFetch<Device>("/devices", { method: "POST", body: newDevice });
 */
export async function apiFetch<T>(
  path: string,
  options: FetchOptions = {},
): Promise<T> {
  const { method = "GET", body, headers = {} } = options;

  const token = localStorage.getItem(CONFIG.tokenKey);

  const requestHeaders: Record<string, string> = {
    "Content-Type": "application/json",
    ...headers,
  };

  if (token) {
    requestHeaders["Authorization"] = `Bearer ${token}`;
  }

  const res = await fetch(`${CONFIG.apiBaseUrl}${path}`, {
    method,
    headers: requestHeaders,
    body: body !== undefined ? JSON.stringify(body) : undefined,
  });

  return handleResponse<T>(res);
}
