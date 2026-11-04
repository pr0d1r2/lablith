# SPEC

## §G GOAL

`src/cache/read` — job reads a `## caches` cache: `NIX_CONFIG` lines, `cache-read.sh`.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
up|src/cache|`## caches` on GitLab: read, push, verify, secrets
self|src/cache/read|job reads a `## caches` cache: `NIX_CONFIG` lines, `cache-read.sh`

## §I INTERFACES

- read: url & key literal → `NIX_CONFIG` lines (= `nix_config()`); ∃ secret → 1st line after runner check `export NIX_CONFIG="$NIX_CONFIG$(bash scripts/lablith/cache-read.sh <url> <key>)"`, arg = literal | `secret:NAME`.

## §V INVARIANTS

V81: read secret → ⊥ in `NIX_CONFIG`; `cache-read.sh` prints both setting lines iff url & key both non-empty, else nothing ∴ pipeline w/o protected vars (unprotected ref, fork MR) runs w/o cache, ⊥ empty substituter

## §T TASKS

id|status|task|cites
T74|.|split from `src/cache` at its token ceiling: spec now, `nix_config` & `read_line` → `read/` after T75; ⊥ behaviour change|V81
