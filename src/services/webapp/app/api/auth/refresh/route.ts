import { cookies } from 'next/headers'
import { NextResponse } from 'next/server'
import { API_URL } from '@/lib/config'
import {
    REFRESH_COOKIE_NAME,
    clearAuthCookies,
    decodeTokenPayload,
    setAuthCookies,
} from '@/lib/auth-cookies'

interface UpstreamAuthBody {
    accessToken: string
    refreshToken: string
}

export async function POST() {
    const cookieStore = await cookies()
    const refreshToken = cookieStore.get(REFRESH_COOKIE_NAME)?.value

    if (!refreshToken) {
        const response = NextResponse.json(
            { message: 'Missing refresh token' },
            { status: 401 },
        )
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
        const response = NextResponse.json(
            { message: 'Unable to refresh token' },
            { status: upstream.status },
        )
        clearAuthCookies(response)
        return response
    }

    const data = (await upstream.json()) as UpstreamAuthBody
    const payload = decodeTokenPayload(data.accessToken)

    if (!payload || !data.refreshToken) {
        const response = NextResponse.json(
            { message: 'Invalid token received from upstream' },
            { status: 500 },
        )
        clearAuthCookies(response)
        return response
    }

    const response = NextResponse.json({
        user: { email: payload.sub, role: payload.role, id: payload.id },
    })

    if (!setAuthCookies(response, data.accessToken, data.refreshToken)) {
        const invalidResponse = NextResponse.json(
            { message: 'Invalid token received from upstream' },
            { status: 500 },
        )
        clearAuthCookies(invalidResponse)
        return invalidResponse
    }

    return response
}
