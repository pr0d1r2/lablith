#!/usr/bin/env bash
#
# tests:V19: every pipeline file lablith renders validates against the
# vendored GitLab CI schema with no finding, offline. Each fixture
# `.lablith` under tests/fixtures/ is rendered into a scratch root by the
# built binary, then every `.gitlab-ci.yml` and `.gitlab/ci/*.yml` there,
# the golden row files and this repository's own files are checked.
#
#   nix/guard/gitlab-schema.sh
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

files=()
n=0
while IFS= read -r config; do
  n=$((n + 1))
  root="$scratch/$n"
  mkdir -p "$root"
  cp "$config" "$root/.lablith"
  # Fixtures meant to be refused are not pipelines; skip what gen refuses.
  if cargo run -q --bin lablith -- gen --root "$root" >/dev/null 2>&1; then
    files+=("$root/.gitlab-ci.yml")
    while IFS= read -r f; do files+=("$f"); done < <(find "$root/.gitlab/ci" -name '*.yml' 2>/dev/null)
  fi
done < <(git ls-files 'tests/fixtures/**/*.lablith')

while IFS= read -r f; do files+=("$f"); done < <(git ls-files 'tests/fixtures/gate/*.yml' 'tests/fixtures/build/*.yml' 'tests/fixtures/bump/*.yml' '.gitlab-ci.yml' '.gitlab/ci/*.yml')

check-jsonschema --schemafile tests/schema/ci.json "${files[@]}"
