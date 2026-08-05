#!/usr/bin/env bash
# PostToolUse hook: CLAUDE.md hard rule — any SQL query change in lib-db requires
# regenerating the .sqlx offline cache, or the CI offline build fails.
# Exit 2 sends the message back to Claude (not to the user as an error).

f=$(jq -r '.tool_response.filePath // .tool_input.file_path // empty')

case "$f" in
  */src/lib/lib-db/build.rs)
    exit 0
    ;;
  */src/lib/lib-db/*.rs)
    echo "lib-db changed: ${f}. CLAUDE.md requires regenerating the SQLx offline cache before finishing this task — run 'just db prepare-offline' (or 'cargo sqlx prepare --workspace') and commit the .sqlx/ changes." >&2
    exit 2
    ;;
esac

exit 0
