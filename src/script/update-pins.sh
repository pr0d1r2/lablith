# shellcheck shell=bash
#
# update-pins.sh [BUILD]
#
# Follow nixpkgs-lock and propose the bump as a merge request (kind
# `update-pins`). Pull model: this project polls; nixpkgs-lock holds no
# cross-project token.
#
#   1. update the `nixpkgs-lock` input; nixpkgs did not move -> exit 0
#   2. branch `pins/nixpkgs-<short rev>` already on origin -> exit 0
#      (src/bump:V33: an earlier run opened that merge request)
#   3. build BUILD (default `.#default`) against the new pin: a bump that
#      breaks the build is found here, not after merge
#   4. `lablith pin` and `lablith gen`, so image pins and a new lablith
#      arrive through the same reviewed merge request ($LABLITH names
#      the command)
#   5. commit, push and open the merge request with open-mr.sh beside
#      this script
set -euo pipefail

build="${1:-.#default}"
lablith="${LABLITH:-lablith}"
here="$(dirname "${BASH_SOURCE[0]}")"

# The nixpkgs the root follows through nixpkgs-lock, not the node named
# `nixpkgs`: once another input brings its own nixpkgs-lock, nix renames
# ours (`nixpkgs-lock_2` -> `nixpkgs_2`) and `nixpkgs` is the other's. An
# input is a node name or a follows path from the root.
nixpkgs_rev() {
  # shellcheck disable=SC2016 # ${...} is nix interpolation, not bash's
  nix eval --raw --impure --expr '
    let
      lock = builtins.fromJSON (builtins.readFile ./flake.lock);
      input = name: key: resolve lock.nodes.${name}.inputs.${key};
      resolve = ref: if builtins.isList ref then follow lock.root ref else ref;
      follow = name: path:
        if path == [ ] then name
        else follow (input name (builtins.head path)) (builtins.tail path);
    in
    lock.nodes.${input (input lock.root "nixpkgs-lock") "nixpkgs"}.locked.rev'
}

before="$(nixpkgs_rev)"
nix flake update nixpkgs-lock
after="$(nixpkgs_rev)"
if [ "$before" = "$after" ]; then
  echo "update-pins: nixpkgs still at $after, nothing to do"
  exit 0
fi

short="${after:0:12}"
branch="pins/nixpkgs-$short"
if git ls-remote --exit-code --heads origin "$branch" >/dev/null 2>&1; then
  echo "update-pins: branch $branch already exists, nothing to do"
  exit 0
fi

nix build --no-link --print-build-logs "$build"
"$lablith" pin
"$lablith" gen

title="build: follow nixpkgs-lock to $short"
body="nixpkgs-lock moved \`$before\` -> \`$after\`.

\`$build\` was built against the new pin, and \`lablith pin\` and
\`lablith gen\` ran in this branch, before this merge request was opened."
bash "$here/open-mr.sh" "$branch" "$title" "$body" \
  flake.lock .lablith .gitlab-ci.yml .gitlab/ci scripts/lablith
