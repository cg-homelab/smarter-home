import 'server-only'
import { createHighlighter, type Highlighter } from 'shiki'

const THEMES = { light: 'github-light', dark: 'github-dark' } as const
const LANGUAGES = ['javascript', 'yaml', 'json'] as const

let highlighterPromise: Promise<Highlighter> | null = null

/**
 * Shiki loads its grammars and themes once per process. Caching the promise
 * keeps repeated renders from re-reading them.
 */
function getHighlighter(): Promise<Highlighter> {
    highlighterPromise ??= createHighlighter({
        themes: Object.values(THEMES),
        langs: [...LANGUAGES],
    })

    return highlighterPromise
}

/**
 * Highlights `code` server-side and returns Shiki markup.
 *
 * `defaultColor: false` emits `--shiki-light` and `--shiki-dark` custom
 * properties instead of baked-in colours, so `globals.css` can switch themes
 * with the `.dark` class without shipping a highlighter to the browser.
 */
export async function highlightCode(
    code: string,
    language: string,
): Promise<string> {
    const highlighter = await getHighlighter()
    const loaded = highlighter.getLoadedLanguages()
    const lang = loaded.includes(language) ? language : 'javascript'

    return highlighter.codeToHtml(code, {
        lang,
        themes: THEMES,
        defaultColor: false,
    })
}
