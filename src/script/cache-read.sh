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
# pipeline that cannot see protected variables (an unprotected ref, a
# fork merge request) gets no cache line at all, never an empty
# substituter, and runs without the cache.
#
# With NETRC, the resolved url must be `https://` before anything is
# written (src/cache/read:V87); otherwise the script stops naming the
# cache and the variable, never the url. The netrc is copied to a 0600
# file in a fresh directory under the job's temp dir
# ($RUNNER_TEMP_PROJECT_DIR, where GitLab keeps File variables, else
# $TMPDIR), and `netrc-file` names that file. The values go to stdout
# for the job's NIX_CONFIG only; nothing is logged.
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
  echo "cache-read: cache $cache: its url is not https; refusing to write the netrc from $netrc" >&2
  exit 1
  ;;
esac

umask 077
dir="$(mktemp -d "${RUNNER_TEMP_PROJECT_DIR:-${TMPDIR:-/tmp}}/lablith-netrc.XXXXXX")"
cp "$file" "$dir/netrc"
chmod 600 "$dir/netrc"
printf '\nextra-substituters = %s\nextra-trusted-public-keys = %s\nnetrc-file = %s\n' \
  "$url" "$key" "$dir/netrc"
