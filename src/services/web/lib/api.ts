import "server-only";
import { cookies } from "next/headers";
import { API_URL } from "./config";
import type { ApiError } from "@/types/error";

export type { ApiError };

export async function handleResponse<T>(res: Response): Promise<T> {
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
  if (res.status === 204 || res.headers.get("content-length") === "0") {
    return undefined as T;
  }
  return res.json() as Promise<T>;
}

/**
 * Server-side fetch wrapper. Reads the JWT from the httpOnly session cookie
 * and attaches it as a Bearer token. Supports Next.js cache tags for
 * fine-grained cache invalidation via revalidateTag().
 */
export async function apiFetch<T>(
  path: string,
  options: RequestInit & { tags?: string[]; revalidate?: number } = {},
): Promise<T> {
  const cookieStore = await cookies();
  const token = cookieStore.get("__session")?.value;

  const { tags, revalidate, ...fetchOptions } = options;

  const hasBody =
    fetchOptions.method !== undefined &&
    fetchOptions.method !== "GET" &&
    fetchOptions.body !== undefined;

  const res = await fetch(`${API_URL}${path}`, {
    ...fetchOptions,
    headers: {
      ...(hasBody && { "Content-Type": "application/json" }),
      ...(token && { Authorization: `Bearer ${token}` }),
      ...(fetchOptions.headers as Record<string, string>),
    },
    ...(tags || revalidate !== undefined
      ? {
          next: {
            ...(tags && { tags }),
            ...(revalidate !== undefined && { revalidate }),
          },
        }
      : {}),
  });

  return handleResponse<T>(res);
}
