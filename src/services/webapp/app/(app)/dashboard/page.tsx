import { getDashboardData } from './actions'
import { PowerLineChart } from '@/components/analytics/power-line-chart'

export default async function DashboardPage() {
    const { home, metrics } = await getDashboardData()

    return (
        <div className="flex flex-col gap-6 p-4 sm:p-6 lg:p-8">
            <div className="flex flex-col sm:flex-row sm:items-center gap-1">
                <h1 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                    Smarter Home
                </h1>
                <p className="text-xs text-gray-500 dark:text-gray-400 sm:ml-3">
                    Energy analytics dashboard
                </p>
            </div>
            <PowerLineChart metrics={metrics} homeName={home?.name ?? null} />
        </div>
    )
}
