# Webapp Agent Addendum

This directory follows repository-wide rules from `AGENTS.md` at repo root.

## Local rules

- Validate frontend changes with:
    - `bun run lint`
    - `bunx tsc --noEmit`
- Keep backend communication in server functions and `src/services/webapp/lib/api.ts`.
- Prefer shared and reusable components for destructive UI interactions.

## Next.js version caution

This repository uses a modern Next.js version with breaking changes between major versions.
Read relevant docs in `node_modules/next/dist/docs/` when uncertain.
