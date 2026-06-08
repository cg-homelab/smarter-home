import { cookies } from 'next/headers'
import { NextResponse } from 'next/server'
import { API_URL } from '@/lib/config'
import {
    ACCESS_COOKIE_NAME,
    REFRESH_COOKIE_NAME,
    clearAuthCookies,
    decodeTokenPayload,
    isAccessTokenExpired,
    setAuthCookies,
} from '@/lib/auth-cookies'

interface UpstreamAuthBody {
    accessToken: string
    refreshToken: string
}

export async function GET() {
    const cookieStore = await cookies()
    const accessToken = cookieStore.get(ACCESS_COOKIE_NAME)?.value
    const refreshToken = cookieStore.get(REFRESH_COOKIE_NAME)?.value

    if (accessToken && !isAccessTokenExpired(accessToken)) {
        const payload = decodeTokenPayload(accessToken)
        if (payload) {
            return NextResponse.json({
                user: {
                    email: payload.sub,
                    role: payload.role,
                    id: payload.id,
                },
            })
        }
    }

    if (!refreshToken) {
        const response = NextResponse.json({ user: null })
        clearAuthCookies(response)
        return response
    }

    const upstream = await fetch(`${API_URL}/auth/refresh`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ refreshToken }),
        cache: 'no-store',
    })

    if (!upstream.ok) {
        const response = NextResponse.json({ user: null })
        clearAuthCookies(response)
        return response
    }

    const data = (await upstream.json()) as UpstreamAuthBody
    const payload = decodeTokenPayload(data.accessToken)

    if (!payload || !data.refreshToken) {
        const response = NextResponse.json({ user: null })
        clearAuthCookies(response)
        return response
    }

    const response = NextResponse.json({
        user: { email: payload.sub, role: payload.role, id: payload.id },
    })

    if (!setAuthCookies(response, data.accessToken, data.refreshToken)) {
        const invalidResponse = NextResponse.json({ user: null })
        clearAuthCookies(invalidResponse)
        return invalidResponse
    }

    return response
}
