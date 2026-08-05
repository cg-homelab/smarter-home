/** Smart home platforms we publish ingest examples for. */
export type IntegrationPlatform = 'homey' | 'home-assistant'

/** A single copy-pasteable ingest script shown on the integrations page. */
export interface ScriptExample {
    /** URL-safe identifier, also used as the React key and heading anchor. */
    id: string
    title: string
    description: string
    method: 'POST'
    /** API path the script sends to, e.g. `/devices/metrics`. */
    path: string
    /** Shiki language id used to highlight `code`. */
    language: string
    code: string
    /** Platform specific caveats rendered underneath the code block. */
    notes?: string[]
}

/** Presentation metadata for one platform tab. */
export interface PlatformMeta {
    label: string
    description: string
    /** Ordered steps to get from a stock hub to a running script. */
    setup: string[]
}
