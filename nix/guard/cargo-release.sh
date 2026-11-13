#!/usr/bin/env bash
#
# The release hook must be available in the same dev shell as the gate. The
# package is held by the nix-cargo-release overlay until nixpkgs catches up.
#
#   nix/guard/cargo-release.sh
set -euo pipefail

if ! command -v cargo-release >/dev/null 2>&1; then
  echo "cargo-release: cargo-release is missing from the dev shell" >&2
  exit 1
fi
