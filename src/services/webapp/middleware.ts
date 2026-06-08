import { NextRequest, NextResponse } from 'next/server'
import {
    ACCESS_COOKIE_NAME,
    REFRESH_COOKIE_NAME,
    clearAuthCookies,
    isAccessTokenExpired,
    setAuthCookies,
} from '@/lib/auth-cookies'

interface UpstreamAuthBody {
    accessToken: string
    refreshToken: string
}

const API_URL = process.env.API_URL ?? 'http://localhost:3001'

export async function middleware(request: NextRequest) {
    const accessToken = request.cookies.get(ACCESS_COOKIE_NAME)?.value
    const refreshToken = request.cookies.get(REFRESH_COOKIE_NAME)?.value

    if (accessToken && !isAccessTokenExpired(accessToken)) {
        return NextResponse.next()
    }

    if (!refreshToken) {
        const response = NextResponse.next()
        clearAuthCookies(response)
        return response
    }

    const upstream = await fetch(`${API_URL}/auth/refresh`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ refreshToken }),
        cache: 'no-store',
    }).catch(() => null)

    if (!upstream || !upstream.ok) {
        const response = NextResponse.next()
        clearAuthCookies(response)
        return response
    }

    const body = (await upstream.json()) as UpstreamAuthBody
    if (!body.accessToken || !body.refreshToken) {
        const response = NextResponse.next()
        clearAuthCookies(response)
        return response
    }

    const response = NextResponse.next()

    if (!setAuthCookies(response, body.accessToken, body.refreshToken)) {
        clearAuthCookies(response)
        return response
    }

    return response
}

export const config = {
    matcher: ['/dashboard/:path*', '/homes/:path*', '/about/:path*'],
}
