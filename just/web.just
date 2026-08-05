# Webapp recipes. All run inside src/services/webapp.
# `check` is read-only; `fmt` and `fix` write files.

set working-directory := '../src/services/webapp'
set windows-shell := ['powershell.exe', '-NoLogo', '-Command']

# Show web recipes
[group('web')]
help:
    @just --list web

# Install webapp dependencies
[group('web')]
install:
    bun install

# Start the Next.js development server
[group('web')]
dev:
    bun run dev

# Lint the webapp (read-only)
[group('web')]
lint:
    bun run lint

# TypeScript type check (read-only)
[group('web')]
typecheck:
    bunx tsc --noEmit

# Format with prettier (writes files)
[group('web')]
fmt:
    bun run format

# Verify prettier formatting without writing files
[group('web')]
fmt-check:
    bunx prettier --check .

# Auto-format and auto-fix lint issues (writes files)
[group('web')]
fix:
    bun run fix

# Read-only webapp gate, mirrors validate-frontend.yml
[group('web')]
check: lint typecheck
