import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from '@/components/ui/card'

export default function AboutPage() {
    return (
        <div className="space-y-6 p-4 sm:p-6 lg:p-8">
            <Card>
                <CardHeader>
                    <CardTitle>About Smarter Home</CardTitle>
                    <CardDescription>
                        Energy monitoring and analytics for your home
                    </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                    <div>
                        <h3 className="font-semibold text-sm mb-2">
                            What is Smarter Home?
                        </h3>
                        <p className="text-sm text-muted-foreground">
                            Smarter Home is an energy monitoring and analytics
                            system that helps you understand your energy
                            consumption patterns and make smarter choices about
                            your energy usage.
                        </p>
                    </div>

                    <div>
                        <h3 className="font-semibold text-sm mb-2">Features</h3>
                        <ul className="text-sm text-muted-foreground space-y-1 list-disc list-inside">
                            <li>Real-time energy consumption tracking</li>
                            <li>Historical data analysis and trends</li>
                            <li>Interactive charts and visualizations</li>
                            <li>
                                TimescaleDB-powered time-series data storage
                            </li>
                        </ul>
                    </div>

                    <div>
                        <h3 className="font-semibold text-sm mb-2">
                            Technology Stack
                        </h3>
                        <ul className="text-sm text-muted-foreground space-y-1 list-disc list-inside">
                            <li>Backend: Rust with Axum framework</li>
                            <li>Frontend: Next.js with App Router</li>
                            <li>Database: TimescaleDB (PostgreSQL)</li>
                            <li>UI: TailwindCSS &amp; Shadcn UI</li>
                            <li>Charts: Recharts</li>
                        </ul>
                    </div>
                </CardContent>
            </Card>
        </div>
    )
}
