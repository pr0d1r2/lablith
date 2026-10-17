#!/usr/bin/env bash
#
# V20 (nix:T8, paradigm D40): no private name of the self-hosted GitLab
# instance -- runner tag, image registry, host, project path -- reaches a
# committed file or commit message. The names live in a local, untracked
# `.private/denylist` (one per line, `#` comments), the paradigm's
# convention. Without that file there is nothing to compare, and the step
# passes: CI never sees the list.
#
# Output names the file and line numbers only, never the matched entry, so
# a CI or hook log cannot leak what the list holds (paradigm dev:V2).
#
#   nix/guard/private-names.sh FILE...
set -euo pipefail

list="$(git rev-parse --show-toplevel)/.private/denylist"
[ -f "$list" ] || exit 0
patterns="$(grep -v '^[[:space:]]*#' "$list" | sed '/^[[:space:]]*$/d' || true)"
[ -n "$patterns" ] || exit 0

found=0
for file in "$@"; do
  # A staged deletion or a binary holds no text to leak.
  [ -f "$file" ] || continue
  grep -Iq . "$file" || continue
  lines="$(grep -n -i -F -f <(printf '%s\n' "$patterns") "$file" | cut -d: -f1 | paste -sd, - || true)"
  if [ -n "$lines" ]; then
    echo "private-names: ${file} line(s) ${lines} hold a name from .private/denylist (V20)." >&2
    found=1
  fi
done
if [ "$found" -ne 0 ]; then
  echo "  fix: replace the name with a neutral one; a private repository overrides defaults in its own .lablith." >&2
fi
exit "$found"
