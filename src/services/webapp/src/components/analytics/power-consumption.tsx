"use client"

import * as React from "react"
import { Area, AreaChart, CartesianGrid, XAxis } from "recharts"

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import {
  type ChartConfig,
  ChartContainer,
  ChartLegend,
  ChartLegendContent,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

export const description = "An interactive area chart"

const chartData = [
    { date: "2024-04-01", hjemme: 222, hytta: 150 },
    { date: "2024-04-02", hjemme: 97, hytta: 180 },
    { date: "2024-04-03", hjemme: 167, hytta: 120 },
    { date: "2024-04-04", hjemme: 242, hytta: 260 },
    { date: "2024-04-05", hjemme: 373, hytta: 290 },
    { date: "2024-04-06", hjemme: 301, hytta: 340 },
    { date: "2024-04-07", hjemme: 245, hytta: 180 },
    { date: "2024-04-08", hjemme: 409, hytta: 320 },
    { date: "2024-04-09", hjemme: 59, hytta: 110 },
    { date: "2024-04-10", hjemme: 261, hytta: 190 },
    { date: "2024-04-11", hjemme: 327, hytta: 350 },
    { date: "2024-04-12", hjemme: 292, hytta: 210 },
    { date: "2024-04-13", hjemme: 342, hytta: 380 },
    { date: "2024-04-14", hjemme: 137, hytta: 220 },
    { date: "2024-04-15", hjemme: 120, hytta: 170 },
    { date: "2024-04-16", hjemme: 138, hytta: 190 },
    { date: "2024-04-17", hjemme: 446, hytta: 360 },
    { date: "2024-04-18", hjemme: 364, hytta: 410 },
    { date: "2024-04-19", hjemme: 243, hytta: 180 },
    { date: "2024-04-20", hjemme: 89, hytta: 150 },
    { date: "2024-04-21", hjemme: 137, hytta: 200 },
    { date: "2024-04-22", hjemme: 224, hytta: 170 },
    { date: "2024-04-23", hjemme: 138, hytta: 230 },
    { date: "2024-04-24", hjemme: 387, hytta: 290 },
    { date: "2024-04-25", hjemme: 215, hytta: 250 },
    { date: "2024-04-26", hjemme: 75, hytta: 130 },
    { date: "2024-04-27", hjemme: 383, hytta: 420 },
    { date: "2024-04-28", hjemme: 122, hytta: 180 },
    { date: "2024-04-29", hjemme: 315, hytta: 240 },
    { date: "2024-04-30", hjemme: 454, hytta: 380 },
    { date: "2024-05-01", hjemme: 165, hytta: 220 },
    { date: "2024-05-02", hjemme: 293, hytta: 310 },
    { date: "2024-05-03", hjemme: 247, hytta: 190 },
    { date: "2024-05-04", hjemme: 385, hytta: 420 },
    { date: "2024-05-05", hjemme: 481, hytta: 390 },
    { date: "2024-05-06", hjemme: 498, hytta: 520 },
    { date: "2024-05-07", hjemme: 388, hytta: 300 },
    { date: "2024-05-08", hjemme: 149, hytta: 210 },
    { date: "2024-05-09", hjemme: 227, hytta: 180 },
    { date: "2024-05-10", hjemme: 293, hytta: 330 },
    { date: "2024-05-11", hjemme: 335, hytta: 270 },
    { date: "2024-05-12", hjemme: 197, hytta: 240 },
    { date: "2024-05-13", hjemme: 197, hytta: 160 },
    { date: "2024-05-14", hjemme: 448, hytta: 490 },
    { date: "2024-05-15", hjemme: 473, hytta: 380 },
    { date: "2024-05-16", hjemme: 338, hytta: 400 },
    { date: "2024-05-17", hjemme: 499, hytta: 420 },
    { date: "2024-05-18", hjemme: 315, hytta: 350 },
    { date: "2024-05-19", hjemme: 235, hytta: 180 },
    { date: "2024-05-20", hjemme: 177, hytta: 230 },
    { date: "2024-05-21", hjemme: 82, hytta: 140 },
    { date: "2024-05-22", hjemme: 81, hytta: 120 },
    { date: "2024-05-23", hjemme: 252, hytta: 290 },
    { date: "2024-05-24", hjemme: 294, hytta: 220 },
    { date: "2024-05-25", hjemme: 201, hytta: 250 },
    { date: "2024-05-26", hjemme: 213, hytta: 170 },
    { date: "2024-05-27", hjemme: 420, hytta: 460 },
    { date: "2024-05-28", hjemme: 233, hytta: 190 },
    { date: "2024-05-29", hjemme: 78, hytta: 130 },
    { date: "2024-05-30", hjemme: 340, hytta: 280 },
    { date: "2024-05-31", hjemme: 178, hytta: 230 },
    { date: "2024-06-01", hjemme: 178, hytta: 200 },
    { date: "2024-06-02", hjemme: 470, hytta: 410 },
    { date: "2024-06-03", hjemme: 103, hytta: 160 },
    { date: "2024-06-04", hjemme: 439, hytta: 380 },
    { date: "2024-06-05", hjemme: 88, hytta: 140 },
    { date: "2024-06-06", hjemme: 294, hytta: 250 },
    { date: "2024-06-07", hjemme: 323, hytta: 370 },
    { date: "2024-06-08", hjemme: 385, hytta: 320 },
    { date: "2024-06-09", hjemme: 438, hytta: 480 },
    { date: "2024-06-10", hjemme: 155, hytta: 200 },
    { date: "2024-06-11", hjemme: 92, hytta: 150 },
    { date: "2024-06-12", hjemme: 492, hytta: 420 },
    { date: "2024-06-13", hjemme: 81, hytta: 130 },
    { date: "2024-06-14", hjemme: 426, hytta: 380 },
    { date: "2024-06-15", hjemme: 307, hytta: 350 },
    { date: "2024-06-16", hjemme: 371, hytta: 310 },
    { date: "2024-06-17", hjemme: 475, hytta: 520 },
    { date: "2024-06-18", hjemme: 107, hytta: 170 },
    { date: "2024-06-19", hjemme: 341, hytta: 290 },
    { date: "2024-06-20", hjemme: 408, hytta: 450 },
    { date: "2024-06-21", hjemme: 169, hytta: 210 },
    { date: "2024-06-22", hjemme: 317, hytta: 270 },
    { date: "2024-06-23", hjemme: 480, hytta: 530 },
    { date: "2024-06-24", hjemme: 132, hytta: 180 },
    { date: "2024-06-25", hjemme: 141, hytta: 190 },
    { date: "2024-06-26", hjemme: 434, hytta: 380 },
    { date: "2024-06-27", hjemme: 448, hytta: 490 },
    { date: "2024-06-28", hjemme: 149, hytta: 200 },
    { date: "2024-06-29", hjemme: 103, hytta: 160 },
    { date: "2024-06-30", hjemme: 446, hytta: 400 },
]

const chartConfig = {
  visitors: {
    label: "Visitors",
  },
  hjemme: {
    label: "Hjemme",
    color: "var(--chart-1)",
  },
  hytta: {
    label: "Hytta",
    color: "var(--chart-2)",
  },
} satisfies ChartConfig

export function PowerConsumptionChart() {
  const [timeRange, setTimeRange] = React.useState("90d")

  const filteredData = chartData.filter((item) => {
    const date = new Date(item.date)
    const referenceDate = new Date("2024-06-30")
    let daysToSubtract = 90
    if (timeRange === "30d") {
      daysToSubtract = 30
    } else if (timeRange === "7d") {
      daysToSubtract = 7
    }
    const startDate = new Date(referenceDate)
    startDate.setDate(startDate.getDate() - daysToSubtract)
    return date >= startDate
  })

  return (
    <Card className="pt-0">
      <CardHeader className="flex items-center gap-2 space-y-0 border-b py-5 sm:flex-row">
        <div className="grid flex-1 gap-1">
          <CardTitle>Your power consumption the last 3 months</CardTitle>
          <CardDescription>
            Showing total power consumption over the last 3 months
          </CardDescription>
        </div>
        <Select value={timeRange} onValueChange={setTimeRange}>
          <SelectTrigger
            className="hidden w-[160px] rounded-lg sm:ml-auto sm:flex"
            aria-label="Select a value"
          >
            <SelectValue placeholder="Last 3 months" />
          </SelectTrigger>
          <SelectContent className="rounded-xl">
            <SelectItem value="90d" className="rounded-lg">
              Last 3 months
            </SelectItem>
            <SelectItem value="30d" className="rounded-lg">
              Last 30 days
            </SelectItem>
            <SelectItem value="7d" className="rounded-lg">
              Last 7 days
            </SelectItem>
          </SelectContent>
        </Select>
      </CardHeader>
      <CardContent className="px-2 pt-4 sm:px-6 sm:pt-6">
        <ChartContainer
          config={chartConfig}
          className="aspect-auto h-[250px] w-full"
        >
          <AreaChart data={filteredData}>
            <defs>
              <linearGradient id="fillHjemme" x1="0" y1="0" x2="0" y2="1">
                <stop
                  offset="5%"
                  stopColor="var(--color-hjemme)"
                  stopOpacity={0.8}
                />
                <stop
                  offset="95%"
                  stopColor="var(--color-hjemme)"
                  stopOpacity={0.1}
                />
              </linearGradient>
              <linearGradient id="fillHytta" x1="0" y1="0" x2="0" y2="1">
                <stop
                  offset="5%"
                  stopColor="var(--color-hytta)"
                  stopOpacity={0.8}
                />
                <stop
                  offset="95%"
                  stopColor="var(--color-hytta)"
                  stopOpacity={0.1}
                />
              </linearGradient>
            </defs>
            <CartesianGrid vertical={false} />
            <XAxis
              dataKey="date"
              tickLine={false}
              axisLine={false}
              tickMargin={8}
              minTickGap={32}
              tickFormatter={(value) => {
                const date = new Date(value)
                return date.toLocaleDateString("en-US", {
                  month: "short",
                  day: "numeric",
                })
              }}
            />
            <ChartTooltip
              cursor={false}
              content={
                <ChartTooltipContent
                  labelFormatter={(value) => {
                    return new Date(value).toLocaleDateString("en-US", {
                      month: "short",
                      day: "numeric",
                    })
                  }}
                  indicator="dot"
                />
              }
            />
            <Area
              dataKey="hytta"
              type="natural"
              fill="url(#fillHytta)"
              stroke="var(--color-hytta)"
              stackId="a"
            />
            <Area
              dataKey="hjemme"
              type="natural"
              fill="url(#fillHjemme)"
              stroke="var(--color-hjemme)"
              stackId="a"
            />
            <ChartLegend content={<ChartLegendContent />} />
          </AreaChart>
        </ChartContainer>
      </CardContent>
    </Card>
  )
}
