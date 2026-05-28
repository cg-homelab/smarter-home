// "use client";

import Link from 'next/link'
import {
    Zap,
    BarChart2,
    Home,
    Clock,
    ShieldCheck,
    Info,
    CheckCircle2,
} from 'lucide-react'
import { ModeToggle } from '@/components/theme/mode-toggle'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

const features = [
    {
        icon: Zap,
        title: 'Real-time monitoring',
        description:
            "See your home's energy consumption the moment it happens — no delays, no guessing.",
    },
    {
        icon: BarChart2,
        title: 'Historical analytics',
        description:
            'Spot trends and seasonal patterns with interactive charts backed by time-series data.',
    },
    {
        icon: Home,
        title: 'Multi-home support',
        description:
            'Manage your main house, cabin, or any property from a single dashboard.',
    },
    {
        icon: Clock,
        title: 'Usage history',
        description:
            'Drill into any time period and understand exactly when and where energy was consumed.',
    },
    {
        icon: ShieldCheck,
        title: 'Secure by design',
        description:
            'Every home gets a unique write token. Your data stays yours.',
    },
    {
        icon: CheckCircle2,
        title: 'Open & self-hosted',
        description:
            'Run Smarter Home on your own infrastructure. No vendor lock-in, no subscriptions.',
    },
]

const stack = [
    { label: 'Backend', value: 'Rust · Axum' },
    { label: 'Database', value: 'TimescaleDB (PostgreSQL)' },
    { label: 'Frontend', value: 'Next.js · TailwindCSS' },
    { label: 'UI Library', value: 'shadcn/ui' },
    { label: 'Charts', value: 'Recharts' },
    { label: 'Desktop', value: 'Tauri v2' },
]

// Middleware handles redirecting authenticated users to /dashboard.
// This page is only rendered for unauthenticated visitors.
export default function LandingPage() {
    return (
        <div className="min-h-screen bg-background flex flex-col">
            {/* Top nav */}
            <header className="sticky top-0 z-50 border-b border-border bg-background/80 backdrop-blur-sm supports-[backdrop-filter]:bg-background/60">
                <div className="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8 py-3 flex items-center justify-between gap-4">
                    <span className="text-base font-bold tracking-tight">
                        Smarter Home
                    </span>
                    <div className="flex items-center gap-2">
                        <ModeToggle />
                        <Button variant="ghost" size="sm" asChild>
                            <Link href="/login">Log in</Link>
                        </Button>
                        <Button size="sm" asChild>
                            <Link href="/register">Sign up</Link>
                        </Button>
                    </div>
                </div>
            </header>

            {/* Main content */}
            <main className="flex-1 mx-auto w-full max-w-5xl px-4 sm:px-6 lg:px-8 py-10">
                <Tabs defaultValue="overview">
                    <div className="flex justify-center mb-8">
                        <TabsList>
                            <TabsTrigger value="overview">Overview</TabsTrigger>
                            <TabsTrigger value="about">
                                <Info className="size-3.5" />
                                About
                            </TabsTrigger>
                        </TabsList>
                    </div>

                    {/* ── Overview tab ── */}
                    <TabsContent value="overview">
                        {/* Hero */}
                        <section className="flex flex-col items-center text-center gap-5 py-12">
                            <div className="inline-flex items-center gap-1.5 rounded-full border border-border bg-muted px-3 py-1 text-xs font-medium text-muted-foreground">
                                <Zap className="size-3" />
                                Self-hosted energy intelligence
                            </div>
                            <h1 className="text-4xl sm:text-5xl font-bold tracking-tight text-foreground leading-tight max-w-2xl">
                                Know exactly where your energy goes
                            </h1>
                            <p className="text-muted-foreground text-base sm:text-lg max-w-xl">
                                Smarter Home turns raw power readings into
                                clear, actionable insights — so you can cut
                                waste, lower bills, and live more sustainably.
                            </p>
                            <div className="flex flex-wrap justify-center gap-3 mt-2">
                                <Button size="lg" asChild>
                                    <Link href="/register">
                                        Get started for free
                                    </Link>
                                </Button>
                                <Button size="lg" variant="outline" asChild>
                                    <Link href="/login">
                                        Log in to dashboard
                                    </Link>
                                </Button>
                            </div>
                        </section>

                        {/* Features grid */}
                        <section className="py-10">
                            <h2 className="text-center text-sm font-semibold uppercase tracking-wider text-muted-foreground mb-8">
                                Everything you need
                            </h2>
                            <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                                {features.map(
                                    ({ icon: Icon, title, description }) => (
                                        <Card key={title} className="gap-3">
                                            <CardHeader className="pb-0">
                                                <CardTitle className="flex items-center gap-2 text-sm font-semibold">
                                                    <span className="flex size-7 items-center justify-center rounded-md bg-primary/10 text-primary">
                                                        <Icon className="size-4" />
                                                    </span>
                                                    {title}
                                                </CardTitle>
                                            </CardHeader>
                                            <CardContent>
                                                <p className="text-sm text-muted-foreground">
                                                    {description}
                                                </p>
                                            </CardContent>
                                        </Card>
                                    ),
                                )}
                            </div>
                        </section>

                        {/* CTA strip */}
                        <section className="mt-6 rounded-xl border bg-muted/50 px-8 py-10 text-center flex flex-col items-center gap-4">
                            <h2 className="text-xl font-bold">
                                Ready to get smarter?
                            </h2>
                            <p className="text-sm text-muted-foreground max-w-md">
                                Create a free account in seconds and start
                                monitoring your first home today.
                            </p>
                            <Button size="lg" asChild>
                                <Link href="/register">Create account</Link>
                            </Button>
                        </section>
                    </TabsContent>

                    {/* ── About tab ── */}
                    <TabsContent value="about">
                        <div className="flex flex-col gap-6 max-w-2xl mx-auto">
                            <div className="text-center py-8">
                                <h2 className="text-3xl font-bold tracking-tight mb-3">
                                    About Smarter Home
                                </h2>
                                <p className="text-muted-foreground">
                                    An open-source energy monitoring and
                                    analytics platform built to run on your own
                                    hardware.
                                </p>
                            </div>

                            <Card>
                                <CardHeader>
                                    <CardTitle className="text-base">
                                        What is Smarter Home?
                                    </CardTitle>
                                </CardHeader>
                                <CardContent className="text-sm text-muted-foreground leading-relaxed">
                                    Smarter Home is a self-hosted system for
                                    tracking and analysing energy usage across
                                    your properties. It collects power readings
                                    from your devices, stores them in a
                                    time-series database, and presents them in
                                    an easy-to-understand dashboard — giving you
                                    full visibility into your consumption habits
                                    without sharing data with any third party.
                                </CardContent>
                            </Card>

                            <Card>
                                <CardHeader>
                                    <CardTitle className="text-base">
                                        Technology stack
                                    </CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <dl className="grid grid-cols-[auto_1fr] gap-x-6 gap-y-2">
                                        {stack.map(({ label, value }) => (
                                            <>
                                                <dt
                                                    key={`dt-${label}`}
                                                    className="text-sm font-medium text-foreground"
                                                >
                                                    {label}
                                                </dt>
                                                <dd
                                                    key={`dd-${label}`}
                                                    className="text-sm text-muted-foreground"
                                                >
                                                    {value}
                                                </dd>
                                            </>
                                        ))}
                                    </dl>
                                </CardContent>
                            </Card>

                            <Card>
                                <CardHeader>
                                    <CardTitle className="text-base">
                                        Key principles
                                    </CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <ul className="flex flex-col gap-2 text-sm text-muted-foreground">
                                        {[
                                            'Privacy-first — all data stays on your own infrastructure',
                                            'Open source — inspect, fork, and contribute freely',
                                            'Lightweight — designed to run on low-power hardware',
                                            'Extensible — structured for new sensors and integrations',
                                        ].map((item) => (
                                            <li
                                                key={item}
                                                className="flex items-start gap-2"
                                            >
                                                <CheckCircle2 className="mt-0.5 size-4 shrink-0 text-primary" />
                                                {item}
                                            </li>
                                        ))}
                                    </ul>
                                </CardContent>
                            </Card>

                            <div className="flex justify-center pb-8">
                                <Button asChild>
                                    <Link href="/register">Get started</Link>
                                </Button>
                            </div>
                        </div>
                    </TabsContent>
                </Tabs>
            </main>

            {/* Footer */}
            <footer className="border-t border-border py-6 text-center text-xs text-muted-foreground">
                © {new Date().getFullYear()} Smarter Home — open-source,
                self-hosted
            </footer>
        </div>
    )
}
