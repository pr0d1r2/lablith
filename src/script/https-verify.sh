# shellcheck shell=bash
#
# https-verify.sh URL NETRC PACKAGE SYSTEM...
#
# Every SYSTEM's `.#packages.<system>.<PACKAGE>` output path answers 200
# from `<URL>/<hash>.narinfo` (kind `build` on an https `## caches`
# cache, src/cache:V84). URL is a literal or `secret:NAME`; NETRC is the
# variable holding the netrc file's path, or `-` for a cache read
# without one. A url that is not `https://` is refused before any
# request. The url is never printed. Arrays expand with the `+` guard,
# so an empty one passes `set -u` on bash 3.2 too.
set -euo pipefail

if [ "$#" -lt 4 ]; then
  echo "usage: https-verify.sh URL NETRC PACKAGE SYSTEM..." >&2
  exit 2
fi
url="$1"
netrc="$2"
package="$3"
shift 3

case "$url" in
secret:*)
  name="${url#secret:}"
  url="${!name:-}"
  ;;
esac
case "$url" in
https://*) ;;
"")
  echo "https-verify: no cache url here" >&2
  exit 1
  ;;
*)
  echo "https-verify: the cache url is not https; refusing to send the netrc" >&2
  exit 1
  ;;
esac
auth=()
if [ "$netrc" != - ] && [ -n "${!netrc:-}" ]; then
  auth=(--netrc-file "${!netrc}")
fi

rc=0
for sys in "$@"; do
  path="$(nix eval --raw ".#packages.$sys.$package")"
  name="${path##*/}"
  hash="${name%%-*}"
  code="$(curl -s -o /dev/null -w '%{http_code}' ${auth[@]+"${auth[@]}"} \
    "$url/$hash.narinfo" || true)"
  echo "https-verify: $sys $hash -> $code"
  if [ "$code" != 200 ]; then
    rc=1
  fi
done
exit "$rc"
