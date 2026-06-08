import 'server-only'
import { cookies } from 'next/headers'
import { API_URL } from './config'
import type { ApiError } from '@/types/error'
import {
    ACCESS_COOKIE_NAME,
    REFRESH_COOKIE_NAME,
    decodeTokenPayload,
} from '@/lib/auth-cookies'

export type { ApiError }

interface UpstreamAuthBody {
    accessToken: string
    refreshToken: string
}

const DEFAULT_REFRESH_TTL_SECS = 30 * 24 * 60 * 60

function refreshTtlSecs(): number {
    const rawValue = Number(process.env.AUTH_REFRESH_TTL_SECS)
    if (!Number.isFinite(rawValue) || rawValue <= 0) {
        return DEFAULT_REFRESH_TTL_SECS
    }

    return Math.floor(rawValue)
}

async function fetchWithToken(
    path: string,
    fetchOptions: RequestInit,
    token: string | undefined,
    tags?: string[],
    revalidate?: number,
): Promise<Response> {
    const hasBody =
        fetchOptions.method !== undefined &&
        fetchOptions.method !== 'GET' &&
        fetchOptions.body !== undefined

    return fetch(`${API_URL}${path}`, {
        ...fetchOptions,
        headers: {
            ...(hasBody && { 'Content-Type': 'application/json' }),
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
    })
}

async function refreshTokenPair(refreshToken: string): Promise<UpstreamAuthBody | null> {
    const refreshRes = await fetch(`${API_URL}/auth/refresh`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ refreshToken }),
        cache: 'no-store',
    })

    if (!refreshRes.ok) return null

    const body = (await refreshRes.json()) as UpstreamAuthBody
    if (!body.accessToken || !body.refreshToken) return null
    return body
}

function setCookiesIfPossible(
    cookieStore: Awaited<ReturnType<typeof cookies>>,
    accessToken: string,
    refreshToken: string,
) {
    const payload = decodeTokenPayload(accessToken)
    if (!payload) return

    try {
        cookieStore.set(ACCESS_COOKIE_NAME, accessToken, {
            httpOnly: true,
            secure: process.env.NODE_ENV === 'production',
            sameSite: 'strict',
            expires: new Date(payload.exp * 1000),
            path: '/',
        })
        cookieStore.set(REFRESH_COOKIE_NAME, refreshToken, {
            httpOnly: true,
            secure: process.env.NODE_ENV === 'production',
            sameSite: 'strict',
            expires: new Date(Date.now() + refreshTtlSecs() * 1000),
            path: '/',
        })
    } catch {
        // Cookie mutation is not available in all server contexts.
    }
}

function clearCookiesIfPossible(cookieStore: Awaited<ReturnType<typeof cookies>>) {
    try {
        cookieStore.delete(ACCESS_COOKIE_NAME)
        cookieStore.delete(REFRESH_COOKIE_NAME)
    } catch {
        // Cookie mutation is not available in all server contexts.
    }
}

export async function handleResponse<T>(res: Response): Promise<T> {
    if (!res.ok) {
        let message = res.statusText
        try {
            const body = await res.text()
            if (body) message = body
        } catch {
            // ignore parse error
        }
        throw { message, status: res.status } satisfies ApiError
    }
    if (res.status === 204 || res.headers.get('content-length') === '0') {
        return undefined as T
    }
    return res.json() as Promise<T>
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
    const cookieStore = await cookies()
    const accessToken = cookieStore.get(ACCESS_COOKIE_NAME)?.value
    const refreshToken = cookieStore.get(REFRESH_COOKIE_NAME)?.value

    const { tags, revalidate, ...fetchOptions } = options

    const initialRes = await fetchWithToken(
        path,
        fetchOptions,
        accessToken,
        tags,
        revalidate,
    )

    if (initialRes.status !== 401 || !refreshToken) {
        return handleResponse<T>(initialRes)
    }

    const refreshed = await refreshTokenPair(refreshToken)

    if (!refreshed) {
        clearCookiesIfPossible(cookieStore)
        return handleResponse<T>(initialRes)
    }

    setCookiesIfPossible(cookieStore, refreshed.accessToken, refreshed.refreshToken)

    const retryRes = await fetchWithToken(
        path,
        fetchOptions,
        refreshed.accessToken,
        tags,
        revalidate,
    )

    if (retryRes.status === 401) {
        clearCookiesIfPossible(cookieStore)
    }

    return handleResponse<T>(retryRes)
}
