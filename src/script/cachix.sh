# shellcheck shell=bash
#
# cachix.sh CACHE COMMAND
#
# Run COMMAND, one shell line (kinds `gate` and `build`), pushing what it
# builds to https://<CACHE>.cachix.org when this pipeline may: a
# CACHIX_AUTH_TOKEN is set (a masked, protected variable, so only on
# protected refs) and the pipeline is not a merge request from a fork
# (src/kind:V14, src/kind:V15). Otherwise COMMAND runs alone and the cache
# is only read. cachix comes from the repository's own locked nixpkgs,
# never from an unpinned registry entry (src/kind:V57).
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: cachix.sh CACHE COMMAND" >&2
  exit 2
fi
cache="$1"
command="$2"

project="${CI_PROJECT_ID:-}"
source="${CI_MERGE_REQUEST_SOURCE_PROJECT_ID:-$project}"
if [ -n "${CACHIX_AUTH_TOKEN:-}" ] && [ "$source" = "$project" ]; then
  exec nix shell --inputs-from . nixpkgs#cachix --command \
    cachix watch-exec "$cache" -- bash -c "$command"
fi
echo "cachix: not pushing to $cache: no token here, or a fork merge request"
exec bash -c "$command"
