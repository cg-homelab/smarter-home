# Webapp Rules

Repository-wide rules live in `CLAUDE.md` at repo root. These are additional and local to `src/services/webapp`.

## Validation

- `bun run lint`
- `bunx tsc --noEmit`

## Rules

- All backend communication goes through server actions and `src/services/webapp/lib/api.ts`. No direct fetches to the API from client components.
- Keep reusable types in `src/services/webapp/types/`.
- Prefer SSR/server-rendered flows and safe hydration patterns.
- Show loading states for async UI with loading UI or skeleton components.
- Reuse `components/ui/skeleton.tsx` for placeholder loading UI before creating custom skeleton variants.
- Prefer shadcn-based UI primitives, added with `bunx --bun shadcn@latest add <component>`.
- Dialogs such as delete/confirm should be reusable components.
- Prefer shared, reusable components for destructive UI interactions.

## Next.js version caution

This repository uses a modern Next.js version with breaking changes between major versions. Read the relevant docs in `node_modules/next/dist/docs/` when uncertain.

## Detailed guidance

- `docs/frontend-guidelines.md`
- `docs/nextjs-best-practices.md`
