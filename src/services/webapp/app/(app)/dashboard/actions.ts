'use server'

import { apiFetch } from '@/lib/api'
import type { Home } from '@/app/(app)/homes/actions'

export interface PowerMetrics {
    homeId: string
    ts: string
    price: number
    power: number
    solarPower: number
    lastMeterConsumption: number
    lastMeterProduction: number
    lastSolarTotal: number
    consumptionSinceMidnight: number
    productionSinceMidnight: number
    solarSinceMidnight: number
    costSinceMidnight: number
    currency: string
}

const USE_MOCK = process.env.DASHBOARD_MOCK === 'true'

/** Generates 30 minutes of synthetic power readings (one per minute). */
function buildMockMetrics(homeId: string): PowerMetrics[] {
    const now = Date.now()
    // Simulate a house that starts consuming ~1500 W, solar kicks in around
    // the midpoint and briefly pushes net power negative.
    const curve = [
        1800, 1650, 1500, 1420, 1300, 1100, 900, 700, 500, 300, 100, -50,
        -200, -350, -500, -620, -700, -580, -400, -200, 0, 150, 350, 500,
        700, 900, 1100, 1300, 1450, 1600,
    ]
    return curve.map((power, i) => {
        const ts = new Date(now - (29 - i) * 60 * 1000).toISOString()
        return {
            homeId,
            ts,
            price: 0.12,
            power,
            solarPower: power < 0 ? Math.abs(power) + 400 : 0,
            lastMeterConsumption: 12345.6 + i * 0.02,
            lastMeterProduction: 4321.0 + (power < 0 ? i * 0.01 : 0),
            lastSolarTotal: 8765.4 + i * 0.015,
            consumptionSinceMidnight: 4.2 + i * 0.025,
            productionSinceMidnight: 1.1 + (power < 0 ? i * 0.01 : 0),
            solarSinceMidnight: 2.3 + i * 0.015,
            costSinceMidnight: 0.5 + i * 0.003,
            currency: 'NOK',
        }
    })
}

export async function getMockDashboardData(): Promise<{
    home: Home | null
    metrics: PowerMetrics[]
}> {
    const home: Home = {
        id: 'mock-home-id',
        name: 'Mock Home',
        address: '1 Example Street',
        writeToken: 'mock-token',
        isFavorite: true,
    }
    return { home, metrics: buildMockMetrics(home.id) }
}

export async function getDashboardData(): Promise<{
    home: Home | null
    metrics: PowerMetrics[]
}> {
    if (USE_MOCK) return getMockDashboardData()

    try {
        const homes = await apiFetch<Home[]>('/home', { tags: ['homes'] })
        const home = homes.find((h) => h.isFavorite) ?? homes[0] ?? null

        if (!home) {
            return { home: null, metrics: [] }
        }

        const endDate = new Date()
        const startDate = new Date(endDate.getTime() - 30 * 60 * 1000)

        const params = new URLSearchParams({
            home_id: home.id,
            start_date: startDate.toISOString(),
            end_date: endDate.toISOString(),
        })

        const metrics = await apiFetch<PowerMetrics[]>(
            `/power/metrics?${params}`,
            { cache: 'no-store' },
        )

        return { home, metrics }
    } catch {
        return { home: null, metrics: [] }
    }
}
