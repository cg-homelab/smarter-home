import { NextResponse } from 'next/server'

export const ACCESS_COOKIE_NAME = '__session'
export const REFRESH_COOKIE_NAME = '__refresh'

export interface JwtPayload {
    sub: string
    role: string
    id: string | null
    exp: number
}

const DEFAULT_REFRESH_TTL_SECS = 30 * 24 * 60 * 60

function refreshTtlSecs(): number {
    const rawValue = Number(process.env.AUTH_REFRESH_TTL_SECS)
    if (!Number.isFinite(rawValue) || rawValue <= 0) {
        return DEFAULT_REFRESH_TTL_SECS
    }

    return Math.floor(rawValue)
}

export function decodeTokenPayload(token: string): JwtPayload | null {
    try {
        const base64 = token.split('.')[1]
        if (!base64) return null

        const normalized = base64.replace(/-/g, '+').replace(/_/g, '/')
        return JSON.parse(atob(normalized)) as JwtPayload
    } catch {
        return null
    }
}

export function isAccessTokenExpired(token: string): boolean {
    const payload = decodeTokenPayload(token)
    if (!payload) return true
    return payload.exp * 1000 < Date.now()
}

function cookieOptions(expires?: Date) {
    return {
        httpOnly: true,
        secure: process.env.NODE_ENV === 'production',
        sameSite: 'strict' as const,
        path: '/',
        ...(expires ? { expires } : {}),
    }
}

export function setAuthCookies(
    response: NextResponse,
    accessToken: string,
    refreshToken: string,
): boolean {
    const payload = decodeTokenPayload(accessToken)
    if (!payload) return false

    response.cookies.set(
        ACCESS_COOKIE_NAME,
        accessToken,
        cookieOptions(new Date(payload.exp * 1000)),
    )
    response.cookies.set(
        REFRESH_COOKIE_NAME,
        refreshToken,
        cookieOptions(new Date(Date.now() + refreshTtlSecs() * 1000)),
    )

    return true
}

export function clearAuthCookies(response: NextResponse) {
    response.cookies.delete(ACCESS_COOKIE_NAME)
    response.cookies.delete(REFRESH_COOKIE_NAME)
}
