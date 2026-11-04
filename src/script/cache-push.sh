# shellcheck shell=bash
#
# cache-push.sh KIND CACHE SECRET BRANCHES COMMAND [INSTALLABLE...]
#
# Run COMMAND, one shell line (kinds `gate` and `build`), pushing to a
# `## caches` cache that names its own push secret (src/cache:V82):
#
#   KIND         `cachix` or `https`
#   CACHE        the cachix cache name, or the https url: a literal or
#                `secret:NAME`, read from the CI/CD variable NAME
#   SECRET       the variable holding the cachix auth token, or the path
#                of the netrc file (a File variable) for an https cache
#   BRANCHES     `any`, or `default`: push only on the default branch
#   INSTALLABLE  what an https push copies, e.g.
#                `.#devShells.x86_64-linux.default.inputDerivation`
#
# The push is skipped, and COMMAND runs alone, for a merge request from a
# fork (the src/kind/rules:V76 condition), when SECRET or the url is
# empty, or off the default branch under `default`. A cachix push runs
# COMMAND under `cachix watch-exec`. An https push first checks that the
# flake has every INSTALLABLE, then copies them with `nix copy` before
# COMMAND, so a gate's inputs are pushed even when the gate fails; the
# netrc is copied to a 0600 file that is removed on exit. A url that is
# not `https://` is refused before the netrc is touched. The push is
# best-effort: a failed copy is a warning and COMMAND still runs, its
# status the script's, as a failed cachix push never fails its job (a
# `build`'s verify-cache job is what catches a missing path). The url is
# never printed.
set -euo pipefail

if [ "$#" -lt 5 ]; then
  echo "usage: cache-push.sh KIND CACHE SECRET BRANCHES COMMAND [INSTALLABLE...]" >&2
  exit 2
fi
kind="$1"
cache="$2"
secret="$3"
branches="$4"
command="$5"
shift 5

case "$cache" in
secret:*)
  name="${cache#secret:}"
  cache="${!name:-}"
  ;;
esac
credential="${!secret:-}"

if [ "$kind" = https ]; then
  case "$cache" in
  "" | https://*) ;;
  *)
    echo "cache-push: the cache url is not https; refusing to send the netrc" >&2
    exit 1
    ;;
  esac
  if [ "$#" -eq 0 ]; then
    echo "usage: cache-push.sh https needs an INSTALLABLE to copy" >&2
    exit 2
  fi
  for installable in "$@"; do
    if ! nix eval --raw "$installable.drvPath" >/dev/null; then
      echo "cache-push: the flake has no $installable; every flake needs it, as the default output a cache keeps" >&2
      exit 1
    fi
  done
fi

why=""
if [ "${CI_MERGE_REQUEST_SOURCE_PROJECT_ID:-}" != "${CI_MERGE_REQUEST_PROJECT_ID:-}" ]; then
  why="a fork merge request"
elif [ -z "$credential" ]; then
  why="no $secret here"
elif [ -z "$cache" ]; then
  why="no cache url here"
elif [ "$branches" = default ] && { [ -z "${CI_COMMIT_BRANCH:-}" ] ||
  [ "${CI_COMMIT_BRANCH:-}" != "${CI_DEFAULT_BRANCH:-}" ]; }; then
  why="not the default branch"
fi
if [ -n "$why" ]; then
  echo "cache-push: not pushing: $why"
  exec bash -c "$command"
fi

case "$kind" in
cachix)
  export CACHIX_AUTH_TOKEN="$credential"
  exec nix shell --inputs-from . nixpkgs#cachix --command \
    cachix watch-exec "$cache" -- bash -c "$command"
  ;;
https)
  umask 077
  dir="$(mktemp -d)"
  trap 'rm -rf "$dir"' EXIT
  cp "$credential" "$dir/netrc"
  chmod 600 "$dir/netrc"
  if ! nix copy --option netrc-file "$dir/netrc" --to "$cache" "$@"; then
    echo "cache-push: warning: the copy to the cache failed; running the command without it" >&2
  fi
  bash -c "$command"
  ;;
*)
  echo "usage: cache-push.sh KIND is cachix or https, not $kind" >&2
  exit 2
  ;;
esac
