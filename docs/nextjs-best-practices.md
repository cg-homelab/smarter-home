# Next.js Best Practices (Webapp)

Scope: src/services/webapp

## Data access boundaries

- Prefer server components and server actions for backend data access.
- Keep API transport logic centralized in src/services/webapp/lib/api.ts.
- Avoid duplicating auth header or cookie handling across components.

## Rendering and UX

- Keep interactive UI in client components and data loading in server paths.
- Provide visible loading states for async data.
- Use route-level loading UI or reusable skeleton components.
- Prefer the shared Skeleton from src/services/webapp/components/ui/skeleton.tsx for consistency.

## Types and contracts

- Keep shared interfaces in src/services/webapp/types/.
- Treat backend response types as contracts and update types alongside API changes.

## UI consistency

- Reuse component primitives from components/ui.
- Prefer shadcn-based additions for new primitives.
- Build reusable modals/dialogs for destructive actions.

## Validation workflow

Before merge:

- bun run lint
- bunx tsc --noEmit
- bun run build (when rendering/data boundaries changed)
