#!/usr/bin/env bash
# PostToolUse hook: CLAUDE.md hard rule — API endpoint changes must keep the
# OpenAPI/Utoipa surface and the auth/backend docs current.
# Exit 2 sends the message back to Claude (not to the user as an error).

f=$(jq -r '.tool_response.filePath // .tool_input.file_path // empty')
[ -n "$f" ] && [ -f "$f" ] || exit 0

case "$f" in
  */src/services/api/src/routes/*.rs)
    # Only nag when the handler surface actually changed, not on helper edits.
    grep -qE '#\[(utoipa::path|axum::debug_handler)\]|async fn|Router::new|\.route\(' "$f" || exit 0
    echo "API route changed: ${f}. CLAUDE.md requires: (1) #[utoipa::path(...)] annotations current for every handler, (2) new paths registered in the OpenApi derive in src/services/api/src/main.rs, (3) docs/auth.md and docs/backend-guidelines.md updated if input, output, or auth behavior changed." >&2
    exit 2
    ;;
esac

exit 0
