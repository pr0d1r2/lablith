# shellcheck shell=bash
#
# runner-system.sh SYSTEM
#
# Stop unless this runner's nix system is SYSTEM, the system the job
# was rendered for (src/kind:V77). It runs first in every nix job, so a
# runner registered with the wrong tags fails before it builds, gates
# or pushes anything for the wrong system. A host that only emulates
# SYSTEM (extra-platforms, binfmt, Rosetta) reports its own system and
# is refused on purpose: a system's runner tags promise a native runner.
# To run SYSTEM emulated anyway, set nix's `system` on that host.
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: runner-system.sh SYSTEM" >&2
  exit 2
fi
want="$1"

found="$(nix eval --raw --impure --expr builtins.currentSystem)"
if [ "$found" != "$want" ]; then
  echo "runner-system: runner nix system is $found, this job needs $want" >&2
  exit 1
fi
