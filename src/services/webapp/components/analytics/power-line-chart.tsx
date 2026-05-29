'use client'

import * as React from 'react'
import {
    AreaChart,
    Area,
    XAxis,
    YAxis,
    CartesianGrid,
    ReferenceLine,
} from 'recharts'
import { AlertCircle, Zap } from 'lucide-react'
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from '@/components/ui/card'
import {
    type ChartConfig,
    ChartContainer,
    ChartTooltip,
    ChartTooltipContent,
} from '@/components/ui/chart'
import type { PowerMetrics } from '@/app/(app)/dashboard/actions'

interface PowerLineChartProps {
    metrics: PowerMetrics[]
    homeName: string | null
}

const chartConfig = {
    power: {
        label: 'Power (W)',
    },
} satisfies ChartConfig

export function PowerLineChart({ metrics, homeName }: PowerLineChartProps) {
    if (!homeName) {
        return (
            <Card>
                <CardContent className="flex flex-col items-center justify-center gap-3 py-16 text-center text-muted-foreground">
                    <Zap className="h-10 w-10 opacity-40" />
                    <div>
                        <p className="text-sm font-medium">
                            No home configured
                        </p>
                        <p className="text-xs">
                            Add a home to start seeing power metrics.
                        </p>
                    </div>
                </CardContent>
            </Card>
        )
    }

    const chartData = metrics.map((m) => ({
        time: m.ts,
        power: Math.round(m.power * 10) / 10,
    }))

    if (chartData.length === 0) {
        return (
            <Card>
                <CardHeader>
                    <CardTitle>Live Power — {homeName}</CardTitle>
                    <CardDescription>Last 30 minutes</CardDescription>
                </CardHeader>
                <CardContent className="flex flex-col items-center justify-center gap-3 py-16 text-center text-muted-foreground">
                    <AlertCircle className="h-10 w-10 opacity-40" />
                    <div>
                        <p className="text-sm font-medium">No data yet</p>
                        <p className="text-xs">
                            No power readings in the last 30 minutes.
                        </p>
                    </div>
                </CardContent>
            </Card>
        )
    }

    const maxPower = Math.max(...chartData.map((d) => d.power))
    const minPower = Math.min(...chartData.map((d) => d.power))
    const range = maxPower - minPower

    // Fraction from the top of the chart where y=0 sits.
    // SVG y-axis: 0 = top (max value), 1 = bottom (min value).
    // Clamped to [0, 1] so all-positive or all-negative datasets
    // still produce a clean single-colour gradient.
    const zeroOffset =
        range > 0 ? Math.min(Math.max(maxPower / range, 0), 1) : 0.5
    const zeroPercent = `${(zeroOffset * 100).toFixed(1)}%`

    const hasNegative = minPower < 0
    const hasPositive = maxPower > 0

    const description =
        hasNegative && !hasPositive
            ? 'Producing power — surplus to grid'
            : hasPositive && !hasNegative
              ? 'Consuming power from grid'
              : 'Green = net production · Red = net consumption'

    return (
        <Card className="pt-0">
            <CardHeader className="flex items-center gap-2 space-y-0 border-b py-5 sm:flex-row">
                <div className="grid flex-1 gap-1">
                    <CardTitle>Live Power — {homeName}</CardTitle>
                    <CardDescription>{description}</CardDescription>
                </div>
            </CardHeader>
            <CardContent className="px-2 pt-4 sm:px-6 sm:pt-6">
                <ChartContainer
                    config={chartConfig}
                    className="aspect-auto h-[250px] w-full"
                >
                    <AreaChart data={chartData}>
                        <defs>
                            {/* Stroke gradient: red above zero, green below */}
                            <linearGradient
                                id="powerStroke"
                                x1="0"
                                y1="0"
                                x2="0"
                                y2="1"
                            >
                                <stop
                                    offset={zeroPercent}
                                    stopColor="var(--destructive)"
                                />
                                <stop
                                    offset={zeroPercent}
                                    stopColor="var(--chart-2)"
                                />
                            </linearGradient>

                            {/* Fill gradient: faint red above zero, faint green below */}
                            <linearGradient
                                id="powerFill"
                                x1="0"
                                y1="0"
                                x2="0"
                                y2="1"
                            >
                                <stop
                                    offset="0%"
                                    stopColor="var(--destructive)"
                                    stopOpacity={0.2}
                                />
                                <stop
                                    offset={zeroPercent}
                                    stopColor="var(--destructive)"
                                    stopOpacity={0.02}
                                />
                                <stop
                                    offset={zeroPercent}
                                    stopColor="var(--chart-2)"
                                    stopOpacity={0.02}
                                />
                                <stop
                                    offset="100%"
                                    stopColor="var(--chart-2)"
                                    stopOpacity={0.2}
                                />
                            </linearGradient>
                        </defs>

                        <CartesianGrid vertical={false} />

                        <XAxis
                            dataKey="time"
                            tickLine={false}
                            axisLine={false}
                            tickMargin={8}
                            minTickGap={48}
                            tickFormatter={(value: string) =>
                                new Date(value).toLocaleTimeString('en-GB', {
                                    hour: '2-digit',
                                    minute: '2-digit',
                                })
                            }
                        />

                        <YAxis
                            tickLine={false}
                            axisLine={false}
                            tickMargin={8}
                            width={58}
                            tickFormatter={(v: number) => `${v}W`}
                        />

                        {/* Zero baseline */}
                        <ReferenceLine
                            y={0}
                            stroke="var(--border)"
                            strokeDasharray="4 4"
                        />

                        <ChartTooltip
                            cursor={{ strokeDasharray: '3 3' }}
                            content={
                                <ChartTooltipContent
                                    labelFormatter={(value) =>
                                        new Date(
                                            value as string,
                                        ).toLocaleTimeString('en-GB', {
                                            hour: '2-digit',
                                            minute: '2-digit',
                                            second: '2-digit',
                                        })
                                    }
                                    formatter={(value) => [
                                        `${value} W`,
                                        'Power',
                                    ]}
                                />
                            }
                        />

                        <Area
                            dataKey="power"
                            type="monotone"
                            stroke="url(#powerStroke)"
                            fill="url(#powerFill)"
                            strokeWidth={2}
                            dot={false}
                            activeDot={{ r: 4 }}
                            baseValue={0}
                        />
                    </AreaChart>
                </ChartContainer>
            </CardContent>
        </Card>
    )
}
