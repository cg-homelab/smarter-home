import { type NextRequest, NextResponse } from 'next/server'
import { API_URL } from '@/lib/config'

function decodeTokenPayload(
    token: string,
): { sub: string; role: string; id: string | null; exp: number } | null {
    try {
        const base64 = token.split('.')[1].replace(/-/g, '+').replace(/_/g, '/')
        return JSON.parse(atob(base64))
    } catch {
        return null
    }
}

export async function POST(request: NextRequest) {
    let body: unknown
    try {
        body = await request.json()
    } catch {
        return NextResponse.json(
            { message: 'Invalid request body' },
            { status: 400 },
        )
    }

    const upstream = await fetch(`${API_URL}/auth/signup`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
    })

    if (!upstream.ok) {
        const error = await upstream.text().catch(() => upstream.statusText)
        return new NextResponse(error, { status: upstream.status })
    }

    const data = (await upstream.json()) as { accessToken: string }
    const payload = decodeTokenPayload(data.accessToken)

    if (!payload) {
        return NextResponse.json(
            { message: 'Invalid token received from upstream' },
            { status: 500 },
        )
    }

    const response = NextResponse.json({
        user: { email: payload.sub, role: payload.role, id: payload.id },
    })

    response.cookies.set('__session', data.accessToken, {
        httpOnly: true,
        secure: process.env.NODE_ENV === 'production',
        sameSite: 'strict',
        expires: new Date(payload.exp * 1000),
        path: '/',
    })

    return response
}
