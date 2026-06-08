import { type NextRequest, NextResponse } from 'next/server'
import { API_URL } from '@/lib/config'
import { decodeTokenPayload, setAuthCookies } from '@/lib/auth-cookies'

interface UpstreamAuthBody {
    accessToken: string
    refreshToken: string
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

    const upstream = await fetch(`${API_URL}/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
    })

    if (!upstream.ok) {
        const error = await upstream.text().catch(() => upstream.statusText)
        return new NextResponse(error, { status: upstream.status })
    }

    const data = (await upstream.json()) as UpstreamAuthBody
    const payload = decodeTokenPayload(data.accessToken)

    if (!payload || !data.refreshToken) {
        return NextResponse.json(
            { message: 'Invalid token received from upstream' },
            { status: 500 },
        )
    }

    const response = NextResponse.json({
        user: { email: payload.sub, role: payload.role, id: payload.id },
    })

    if (!setAuthCookies(response, data.accessToken, data.refreshToken)) {
        return NextResponse.json(
            { message: 'Invalid token received from upstream' },
            { status: 500 },
        )
    }

    return response
}
