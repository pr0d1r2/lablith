# shellcheck shell=bash
#
# upstream.sh SCRIPT BRANCH_PREFIX
#
# Poll an upstream release and propose the bump as a merge request (kind
# `upstream`). SCRIPT is the project's own bump script, run as
# `./SCRIPT`; it updates the tree and prints `NEW_VERSION=<version>` when
# there is a new release. Nothing printed, no merge request. The branch is
# BRANCH_PREFIX followed by the version; open-mr.sh beside this script
# keeps the run idempotent (src/bump:V33).
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: upstream.sh SCRIPT BRANCH_PREFIX" >&2
  exit 2
fi
script="$1"
prefix="$2"
here="$(dirname "${BASH_SOURCE[0]}")"

out="$("./$script")"
printf '%s\n' "$out"
version="$(printf '%s\n' "$out" | sed -n 's/^NEW_VERSION=//p' | tail -n 1)"
if [ -z "$version" ]; then
  echo "upstream: $script printed no NEW_VERSION, nothing to do"
  exit 0
fi

bash "$here/open-mr.sh" "$prefix$version" "build: bump to $version" \
  "Bumped by $script, which ran before this merge request was opened."
