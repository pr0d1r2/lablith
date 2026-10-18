# shellcheck shell=bash
#
# Test double for git, nix, curl and lablith (src/script:T20). The unit
# tests copy it under each tool's name, first on PATH, with a shebang
# naming the bash they found. Every call is appended to $STUB_LOG as
# "<tool> <args>"; the STUB_* variables below choose the answers.
set -euo pipefail

tool="${0##*/}"
printf '%s %s\n' "$tool" "$*" >>"$STUB_LOG"

case "$tool $*" in
"git ls-remote"* | "git -c "*" ls-remote "*)
  # --exit-code: 0 when the branch exists, 2 when it does not.
  if [ "${STUB_BRANCH_EXISTS:-}" = 1 ]; then exit 0; fi
  exit 2
  ;;
"git diff --cached --quiet"*)
  if [ "${STUB_NOTHING_STAGED:-}" = 1 ]; then exit 0; fi
  exit 1
  ;;
"nix flake update"*)
  : >"$STUB_LOG.updated"
  # STUB_LOCK_AFTER: the flake.lock the update leaves behind.
  if [ -n "${STUB_LOCK_AFTER:-}" ]; then cp "$STUB_LOCK_AFTER" flake.lock; fi
  ;;
"nix eval --raw --impure"*)
  # STUB_NIX: the real nix, evaluating the expression on ./flake.lock.
  if [ -n "${STUB_NIX:-}" ]; then exec "$STUB_NIX" "$@"; fi
  # Otherwise the nixpkgs rev: STUB_REV_BEFORE until `flake update`,
  # then AFTER.
  if [ -e "$STUB_LOG.updated" ]; then
    echo "${STUB_REV_AFTER:-}"
  else
    echo "${STUB_REV_BEFORE:-}"
  fi
  ;;
"nix eval --raw .#packages."*)
  attr="${3#.#packages.}"
  sys="${attr%%.*}"
  printf '/nix/store/%s-%s\n' "${sys//-/}" "${attr#*.}"
  ;;
"nix build"*)
  if [ "${STUB_BUILD_FAILS:-}" = 1 ]; then exit 1; fi
  ;;
"curl "*)
  url="${*: -1}"
  case "$url" in
  *"${STUB_CURL_MISSING:-no-such-hash}"*) printf '404' ;;
  *) printf '200' ;;
  esac
  ;;
esac
