import Link from 'next/link'
import { KeyRound } from 'lucide-react'
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { CodeBlock } from '@/components/integrations/code-block'
import {
    isIntegrationPlatform,
    platformMeta,
    platformOrder,
    scriptExamples,
} from '@/lib/integrations/scripts'
import type { IntegrationPlatform } from '@/types/integrations'

export const metadata = {
    title: 'Integrations',
    description:
        'Example scripts for sending smart home data to the Smarter Home API.',
}

interface IntegrationsPageProps {
    searchParams: Promise<{ platform?: string }>
}

export default async function IntegrationsPage({
    searchParams,
}: IntegrationsPageProps) {
    const { platform: requested } = await searchParams
    const platform: IntegrationPlatform = isIntegrationPlatform(requested)
        ? requested
        : 'homey'

    const meta = platformMeta[platform]
    const examples = scriptExamples[platform]

    return (
        <div className="space-y-6 p-4 sm:p-6 lg:p-8">
            <Card>
                <CardHeader>
                    <CardTitle>Integrations</CardTitle>
                    <CardDescription>
                        Example scripts that push data from your smart home hub
                        into Smarter Home
                    </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                    <p className="text-sm text-muted-foreground">
                        Every script below is a starting point: copy it, replace
                        the three values at the top, and schedule it on your
                        hub. The payloads match the ingest endpoints exactly, so
                        anything you send shows up on the dashboard.
                    </p>
                    <div className="flex gap-3 rounded-lg border bg-muted/40 p-4">
                        <KeyRound
                            className="mt-0.5 size-4 shrink-0 text-muted-foreground"
                            aria-hidden
                        />
                        <div className="space-y-1 text-sm">
                            <p className="font-medium">
                                Use the home write token, not your login
                            </p>
                            <p className="text-muted-foreground">
                                Ingest requests authenticate with the write
                                token of the home they write to, and the API
                                rejects any request where the token and{' '}
                                <code className="rounded bg-muted px-1 py-0.5 font-mono text-xs">
                                    homeId
                                </code>{' '}
                                disagree. Both values are on the{' '}
                                <Link
                                    href="/homes"
                                    className="font-medium text-foreground underline underline-offset-4"
                                >
                                    Homes page
                                </Link>
                                . Treat the write token as a secret: it does not
                                expire.
                            </p>
                        </div>
                    </div>
                </CardContent>
            </Card>

            <Tabs value={platform} className="gap-6">
                <TabsList>
                    {platformOrder.map((option) => (
                        <TabsTrigger key={option} value={option} asChild>
                            <Link
                                href={`/integrations?platform=${option}`}
                                scroll={false}
                                // Radix roving focus renders -1 until hydration,
                                // which would leave these links unreachable by
                                // keyboard on the server-rendered page.
                                tabIndex={0}
                            >
                                {platformMeta[option].label}
                            </Link>
                        </TabsTrigger>
                    ))}
                </TabsList>

                <TabsContent value={platform} className="space-y-6">
                    <Card>
                        <CardHeader>
                            <CardTitle>Setting up {meta.label}</CardTitle>
                            <CardDescription>
                                {meta.description}
                            </CardDescription>
                        </CardHeader>
                        <CardContent>
                            <ol className="list-inside list-decimal space-y-1 text-sm text-muted-foreground">
                                {meta.setup.map((step) => (
                                    <li key={step}>{step}</li>
                                ))}
                            </ol>
                        </CardContent>
                    </Card>

                    {examples.map((example) => (
                        <Card key={example.id} id={example.id}>
                            <CardHeader>
                                <div className="flex flex-wrap items-center gap-2">
                                    <CardTitle>{example.title}</CardTitle>
                                    <Badge
                                        variant="secondary"
                                        className="font-mono"
                                    >
                                        {example.method} {example.path}
                                    </Badge>
                                </div>
                                <CardDescription>
                                    {example.description}
                                </CardDescription>
                            </CardHeader>
                            <CardContent className="space-y-4">
                                <CodeBlock
                                    code={example.code}
                                    language={example.language}
                                />
                                {example.notes && (
                                    <ul className="list-inside list-disc space-y-1 text-sm text-muted-foreground">
                                        {example.notes.map((note) => (
                                            <li
                                                key={note}
                                                className="whitespace-pre-line"
                                            >
                                                {note}
                                            </li>
                                        ))}
                                    </ul>
                                )}
                            </CardContent>
                        </Card>
                    ))}
                </TabsContent>
            </Tabs>
        </div>
    )
}
