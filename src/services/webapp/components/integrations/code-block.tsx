import { highlightCode } from '@/lib/highlight'
import { CopyButton } from './copy-button'
import { cn } from '@/lib/utils'

interface CodeBlockProps {
    code: string
    language: string
    className?: string
}

/**
 * Server-rendered, syntax highlighted code block with a copy button.
 *
 * The markup comes from Shiki, which escapes the input, and `code` is static
 * repository content rather than user input.
 */
export async function CodeBlock({ code, language, className }: CodeBlockProps) {
    const html = await highlightCode(code, language)

    return (
        <div className={cn('relative', className)}>
            <div className="absolute top-2 right-2 z-10">
                <CopyButton value={code} />
            </div>
            <div
                className="code-block overflow-hidden rounded-lg border text-xs"
                dangerouslySetInnerHTML={{ __html: html }}
            />
        </div>
    )
}
