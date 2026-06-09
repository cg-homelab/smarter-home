import { cookies } from 'next/headers'
import { NextResponse } from 'next/server'
import { API_URL } from '@/lib/config'
import { REFRESH_COOKIE_NAME, clearAuthCookies } from '@/lib/auth-cookies'

export async function POST() {
    const cookieStore = await cookies()
    const refreshToken = cookieStore.get(REFRESH_COOKIE_NAME)?.value

    if (refreshToken) {
        await fetch(`${API_URL}/auth/logout`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ refreshToken }),
            cache: 'no-store',
        }).catch(() => undefined)
    }

    const response = NextResponse.json({ success: true })
    clearAuthCookies(response)
    return response
}
