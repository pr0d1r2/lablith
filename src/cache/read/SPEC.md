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

- read: url & key literal ∧ ⊥ read netrc → `NIX_CONFIG` lines (= `nix_config()`); else 1st line after runner check `NIX_CONFIG="$NIX_CONFIG$(bash scripts/lablith/cache-read.sh <url> <key> [<R> <name>])"`, url & key arg = literal | `secret:NAME`, R = read netrc variable (`read_netrc()`), name = cache's. assignment ⊥ `export` (`NIX_CONFIG` already exported job variable) ∴ script's exit status stops job; `export` would mask it.
- GitLab variable: read netrc = protected File variable; push netrc (`src/cache:V82`) ⊥ used for reads.

## §V INVARIANTS

V81: read secret → ⊥ in `NIX_CONFIG`; `cache-read.sh` prints both setting lines iff url & key both non-empty, else nothing ∴ pipeline w/o protected vars (unprotected ref, fork MR) runs w/o cache, ⊥ empty substituter

V86: read netrc → `cache-read.sh` prints `netrc-file = <file>` after both setting lines; file = `$R`'s own File variable file (runner writes it in job temp dir, removes it at job end), `chmod 600` after V87 check; ⊥ copy (⊥ `after_script` to remove one) ∴ ⊥ credential outlives job. url, key | `$R` empty (unprotected ref; fork MR stopped earlier by `src/kind/rules:V76`) → prints nothing ∴ ⊥ cache. `https-verify.sh` asks w/ read netrc ∃, else push netrc (`src/cache:V84`); `report` purpose `https cache read (netrc file)`
V87: before handing netrc to nix (chmod, `netrc-file`), `cache-read.sh` checks RESOLVED url starts `https://`; else exit 1, ⊥ `netrc-file`, message names cache & `R`, ⊥ url, ⊥ secret value (pipelith V87)

## §T TASKS

id|status|task|cites
T74|x|split from `src/cache` at its token ceiling: spec, then `nix_config` & `read_line` → `read/` after T75; ⊥ behaviour change|V81
T75|x|pipelith 0.2.2 read netrc: `cache-read.sh` netrc arg, run-time https check, assignment line, verify & `report` w/ read netrc; test per V; fork guard stays 1st rule|V86,V87,`src/kind/rules:V76`
T76|x|review #37: netrc in place, ⊥ copy left on runner|V86
