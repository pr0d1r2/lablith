#!/usr/bin/env bash
#
# tests:V61: the vendored GitLab CI schema is the copy tests/schema/VENDORED.md
# names: its sha256 matches, and the file names the GitLab tag it came from.
#
#   nix/guard/ci-schema-pin.sh
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"
sum="$(sha256sum tests/schema/ci.json | cut -d' ' -f1)"
if ! grep -q "^- \`ci.json\` sha256: $sum\$" tests/schema/VENDORED.md; then
  echo "ci-schema-pin: tests/schema/ci.json (sha256 $sum) is not the copy tests/schema/VENDORED.md names" >&2
  echo "  fix: replace ci.json from the tag VENDORED.md names, or update its tag and sha256 in the same commit" >&2
  exit 1
fi
if ! grep -q '^- GitLab tag: v[0-9]' tests/schema/VENDORED.md; then
  echo "ci-schema-pin: tests/schema/VENDORED.md names no GitLab tag" >&2
  exit 1
fi
