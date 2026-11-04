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
# With NETRC, the resolved url must be `https://` and the netrc free of
# a `default` entry before the netrc is touched (src/cache/read:V87);
# otherwise the script stops naming the cache and the variable, never
# the url or the credential. Then the File variable's own
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

# A `default` entry matches every host, so nix would send its credential
# to any substituter; only `machine` entries scope it to the cache. The
# scan reads tokens as curl does: keywords in any case, a quoted token
# whole (with \-escapes), an unquoted `#` at the start of a token ending
# the line, the token after login, password, machine and macdef a value
# (curl has no `account`, so the token after it is a keyword), and a
# macdef body up to the next blank line. It exits 3 on a `default` entry; any
# status but 0 or 3 is a scan that could not read the file, and is
# refused too.
status=0
awk '
  function split_tokens(line, n, i, c, t, q, esc, in_tok) {
    n = 0; t = ""; q = 0; esc = 0; in_tok = 0
    for (i = 1; i <= length(line); i++) {
      c = substr(line, i, 1)
      if (q) {
        if (esc) { t = t c; esc = 0 }
        else if (c == "\\") esc = 1
        else if (c == "\"") q = 0
        else t = t c
      } else if (c == " " || c == "\t" || c == "\r") {
        if (in_tok) { tok[++n] = t; t = ""; in_tok = 0 }
      } else if (c == "\"" && !in_tok) { q = 1; in_tok = 1 }
      else if (c == "#" && !in_tok) break
      else { t = t c; in_tok = 1 }
    }
    if (in_tok) tok[++n] = t
    return n
  }
  mac { if ($0 ~ /^[[:space:]]*$/) mac = 0; next }
  {
    n = split_tokens($0)
    for (k = 1; k <= n; k++) {
      if (skip) { skip = 0; continue }
      w = tolower(tok[k])
      if (w ~ /^(machine|login|password)$/) skip = 1
      else if (w == "macdef") { skip = 1; mac = 1 }
      else if (w == "default") found = 1
    }
  }
  END { exit found ? 3 : 0 }
' "$file" 2>/dev/null || status=$?
case "$status" in
0) ;;
3)
  echo "cache-read: cache $cache: the netrc in $netrc has a default entry, which would reach every host; use machine entries only" >&2
  exit 1
  ;;
*)
  echo "cache-read: cache $cache: cannot read the netrc in $netrc; refusing to use it" >&2
  exit 1
  ;;
esac
chmod 600 "$file"
printf '\nextra-substituters = %s\nextra-trusted-public-keys = %s\nnetrc-file = %s\n' \
  "$url" "$key" "$file"
