#!/usr/bin/env bash
#
# The package consumers get is tested as built, not only the source tree
# (lablith nix:T33, as actlith and pklith). It builds the flake's lablith with
# nix and runs the built binary through the core loop on a fixture
# repository: detect, seed --init, gen, gen --check, check, and a drifted
# file that gen --check must refuse.
#
#   nix/guard/package-nix.sh
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
unset GIT_DIR GIT_INDEX_FILE GIT_WORK_TREE GIT_PREFIX GIT_OBJECT_DIRECTORY \
  GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_COMMON_DIR

fail() {
  echo "package-nix: $*" >&2
  exit 1
}

# `git+file:` builds from tracked files only; a bare path would copy
# target/ while cargo steps rewrite it (actlith nix:B8).
out="$(nix build "git+file://$root#default" --no-link --print-out-paths)"
lablith="$out/bin/lablith"
[ -x "$lablith" ] || fail "$lablith is not an executable"

expect() {
  local what="$1" want="$2" got="$3"
  [ "$got" = "$want" ] || fail "$what: expected $(printf '%q' "$want"), got $(printf '%q' "$got")"
}

version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)"
expect "lablith --version" "lablith $version" "$("$lablith" --version)"

fixture="$(mktemp -d)"
trap 'rm -rf "$fixture"' EXIT
git -C "$fixture" init -q
printf '{ outputs = _: { }; }\n' >"$fixture/flake.nix"
git -C "$fixture" add -A

run() { (cd "$fixture" && "$lablith" "$@"); }

expect "lablith detect" "ci|gate|-|-|-" "$(run detect)"
run seed --init >/dev/null || fail "seed --init failed on a fresh repository"
run gen >/dev/null || fail "gen failed"
[ -s "$fixture/.gitlab-ci.yml" ] || fail "gen wrote no .gitlab-ci.yml"
[ -s "$fixture/.gitlab/ci/ci.yml" ] || fail "gen wrote no .gitlab/ci/ci.yml"
git -C "$fixture" add -A
run gen --check || fail "gen --check refused what gen just wrote"
run check || fail "check refused a repository gen just wrote"

printf '# edited by hand\n' >>"$fixture/.gitlab/ci/ci.yml"
if run gen --check 2>/dev/null; then
  fail "gen --check passed a hand-edited pipeline file"
fi
