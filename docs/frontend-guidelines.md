# Frontend Guidelines

Scope: src/services/webapp

## Required validation commands

Run these before merging frontend changes:

- bun run lint
- bunx tsc --noEmit

Run this when route logic, rendering behavior, or data boundaries changed:

- bun run build

## Architecture boundaries

- All communication with backend APIs should go through server functions in action files.
- Shared API transport logic must go through src/services/webapp/lib/api.ts.
- Avoid ad-hoc fetch calls scattered in UI components for backend endpoints.

## Type organization

- Keep reusable frontend types in src/services/webapp/types/.
- Keep action-local temporary types near action files only when they are not reused.
- Promote duplicated action-local types into src/services/webapp/types/.

## Rendering, performance, and security

- Prefer server-rendered data paths where possible.
- Use safe hydration boundaries by limiting client components to interactive parts.
- Display loading UI (loading.tsx or skeleton/loading component) when waiting on remote data.
- Use src/services/webapp/components/ui/skeleton.tsx as the default skeleton primitive and compose layout-specific placeholders with className.
- Never expose secrets or privileged tokens to browser-visible state.

## UI component policy

- Prefer shadcn UI components for shared primitives.
- Add components using:
  - bunx --bun shadcn@latest add <component>
- Reuse existing primitives before creating custom one-off variants.

## Reusable destructive actions

- Delete confirmations and similar destructive UX should be reusable components.
- Keep confirm dialogs in shared components and pass text/actions as props.

## API service contract

- src/services/webapp/lib/api.ts is the canonical API service layer.
- New backend request concerns (auth headers, tracing metadata, cache semantics) belong there, not in page components.
