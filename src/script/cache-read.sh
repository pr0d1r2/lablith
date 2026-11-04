# shellcheck shell=bash
#
# cache-read.sh URL KEY [NETRC CACHE]
#
# Print the nix settings that trust a `## caches` cache whose url or key
# is a secret, or that is read with a netrc (src/cache/read:V81, V86),
# for a job to append to its NIX_CONFIG:
#
#   NIX_CONFIG="$NIX_CONFIG$(bash scripts/lablith/cache-read.sh URL KEY)"
#
# The line assigns rather than exports: NIX_CONFIG is already an
# exported job variable, and `export` would hide this script's exit
# status, so a refused url would not stop the job.
#
# URL and KEY are each a literal or `secret:NAME`, read from the CI/CD
# variable NAME. NETRC is the File variable holding the read netrc's
# path and CACHE the cache's name, both given only for a cache read with
# a netrc. The settings are printed only when every value is there: a
# pipeline that cannot see protected variables (an unprotected ref)
# gets no cache line at all, never an empty substituter, and runs
# without the cache. A fork's merge request never gets here: the fork
# guard (src/kind/rules:V76) skips the job.
#
# With NETRC, the resolved url must be `https://` before the netrc is
# touched (src/cache/read:V87); otherwise the script stops naming the
# cache and the variable, never the url. Then the File variable's own
# file, which the runner writes in the job's temp dir and removes when
# the job ends, is made 0600 and `netrc-file` names it. No copy is made:
# nix reads the file after this script exits, so a trap could not
# remove one, and a copy would outlive the job on a shared runner. The
# values go to stdout for the job's NIX_CONFIG only; nothing is logged.
set -euo pipefail

if [ "$#" -ne 2 ] && [ "$#" -ne 4 ]; then
  echo "usage: cache-read.sh URL KEY [NETRC CACHE]" >&2
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
if [ -z "$url" ] || [ -z "$key" ]; then
  exit 0
fi
if [ "$#" -eq 2 ]; then
  printf '\nextra-substituters = %s\nextra-trusted-public-keys = %s\n' \
    "$url" "$key"
  exit 0
fi

netrc="$3"
cache="$4"
file="${!netrc:-}"
if [ -z "$file" ]; then
  exit 0
fi
case "$url" in
https://*) ;;
*)
  echo "cache-read: cache $cache: its url is not https; refusing to use the netrc from $netrc" >&2
  exit 1
  ;;
esac

chmod 600 "$file"
printf '\nextra-substituters = %s\nextra-trusted-public-keys = %s\nnetrc-file = %s\n' \
  "$url" "$key" "$file"
