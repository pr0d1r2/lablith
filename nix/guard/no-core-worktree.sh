#!/usr/bin/env bash
#
# nix:V46: parallel wave workers share no git state. `core.worktree` in
# .git/config is read by every worktree of the repository, so one worker
# that sets it points every other checkout's git at its own tree. It must
# never be set in the shared config.
#
#   nix/guard/no-core-worktree.sh
set -euo pipefail

common="$(git rev-parse --git-common-dir)"
if value="$(git config --file "$common/config" --get core.worktree)"; then
  echo "no-core-worktree: core.worktree is set to ${value} in ${common}/config, which every worktree shares (nix:V46)." >&2
  echo "  fix: git config --file ${common}/config --unset core.worktree" >&2
  exit 1
fi
