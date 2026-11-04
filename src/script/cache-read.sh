# shellcheck shell=bash
#
# cache-read.sh URL KEY
#
# Print the nix settings that trust a `## caches` cache whose url or key
# is a secret (src/cache:V81), for a job to append to its NIX_CONFIG:
#
#   export NIX_CONFIG="$NIX_CONFIG$(bash scripts/lablith/cache-read.sh URL KEY)"
#
# URL and KEY are each a literal or `secret:NAME`, read from the CI/CD
# variable NAME. Both lines are printed only when both values are there:
# a pipeline that cannot see protected variables (an unprotected ref, a
# fork merge request) gets no cache line at all, never an empty
# substituter, and runs without the cache. The values go to stdout for
# the job's NIX_CONFIG only; nothing is logged. A later setting (a
# netrc-file for a cache that needs credentials to read) joins here, and
# with it the cache-push.sh rule: a url that is not `https://` is refused
# before any netrc is used.
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: cache-read.sh URL KEY" >&2
  exit 2
fi

# The value of a literal or `secret:NAME` argument.
value() {
  case "$1" in
  secret:*)
    local name="${1#secret:}"
    printf '%s' "${!name:-}"
    ;;
  *) printf '%s' "$1" ;;
  esac
}

url="$(value "$1")"
key="$(value "$2")"
if [ -n "$url" ] && [ -n "$key" ]; then
  printf '\nextra-substituters = %s\nextra-trusted-public-keys = %s\n' \
    "$url" "$key"
fi
