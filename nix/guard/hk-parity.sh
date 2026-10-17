#!/usr/bin/env bash
# nix:V47 (pklith V35, its B12): hk evaluates hk.pkl with its own Pkl implementation,
# pklr, unless told otherwise. A construct pklr mishandles makes steps vanish
# or lose their command while `pkl eval` still lists them, and the gate then
# passes having run almost nothing. This fails unless both evaluators see the
# same steps, each with a command.
#
#   nix/guard/hk-parity.sh
set -euo pipefail

want="$(pkl eval --format json hk.pkl |
  jq -r '.hooks.check.steps | to_entries[] | select(.value.check != null or .value.check_diff != null) | .key' | sort)"
seen="$(hk check --check --all --plan --json |
  jq -r '.steps[] | select([.reasons[].kind] | index("no_command") | not) | .name' | sort)"

if [ "$want" != "$seen" ]; then
  echo "hk-parity: hk's own Pkl evaluator does not see the steps pkl eval sees (nix:V47):" >&2
  diff <(printf '%s\n' "$want") <(printf '%s\n' "$seen") |
    sed -n 's/^< /  missing in hk: /p; s/^> /  only in hk: /p' >&2
  exit 1
fi
