'use client'

import * as React from 'react'
import { Check, Copy } from 'lucide-react'
import { Button } from '@/components/ui/button'

interface CopyButtonProps {
    value: string
    label?: string
}

/**
 * Copies `value` to the clipboard and confirms with a check icon for two
 * seconds. The only client component on the integrations page.
 */
export function CopyButton({ value, label = 'Copy code' }: CopyButtonProps) {
    const [copied, setCopied] = React.useState(false)

    React.useEffect(() => {
        if (!copied) {
            return
        }

        const timeout = setTimeout(() => setCopied(false), 2000)
        return () => clearTimeout(timeout)
    }, [copied])

    async function handleCopy() {
        try {
            await navigator.clipboard.writeText(value)
            setCopied(true)
        } catch {
            setCopied(false)
        }
    }

    return (
        <Button
            type="button"
            variant="outline"
            size="icon-sm"
            onClick={handleCopy}
            aria-label={copied ? 'Copied' : label}
        >
            {copied ? <Check aria-hidden /> : <Copy aria-hidden />}
        </Button>
    )
}
