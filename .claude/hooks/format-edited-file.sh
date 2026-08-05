#!/usr/bin/env bash
# PostToolUse hook: format the file Claude just edited.
#   *.rs                -> rustfmt, using the edition of the owning crate
#   webapp *.ts/tsx/css -> prettier from src/services/webapp/node_modules
#   webapp *.ts/tsx     -> eslint --fix as well, so CI lint failures surface at edit time
# Always exits 0: a formatting failure must never block the edit.

f=$(jq -r '.tool_response.filePath // .tool_input.file_path // empty')
[ -n "$f" ] && [ -f "$f" ] || exit 0

root="${CLAUDE_PROJECT_DIR:-$(pwd)}"

case "$f" in
  *.rs)
    # Walk up to the owning crate's Cargo.toml to read its edition.
    ed=""
    d=$(dirname "$f")
    while [ "$d" != "/" ] && [ -z "$ed" ]; do
      if [ -f "$d/Cargo.toml" ]; then
        ed=$(grep -m1 '^edition' "$d/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')
      fi
      d=$(dirname "$d")
    done
    rustfmt --edition "${ed:-2021}" "$f"
    ;;
  */src/services/webapp/*.ts | */src/services/webapp/*.tsx | */src/services/webapp/*.css)
    "$root/src/services/webapp/node_modules/.bin/prettier" --write --ignore-unknown "$f" >/dev/null
    case "$f" in
      *.ts | *.tsx)
        # eslint flat config resolves from cwd, so run from the webapp root.
        (cd "$root/src/services/webapp" && ./node_modules/.bin/eslint --fix "$f") >/dev/null 2>&1
        ;;
    esac
    ;;
esac

exit 0
