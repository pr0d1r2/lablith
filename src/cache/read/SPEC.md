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

- read: url & key literal ∧ ⊥ read netrc → `NIX_CONFIG` lines (= `nix_config()`); else 1st line after runner check `NIX_CONFIG="$NIX_CONFIG$(bash scripts/lablith/cache-read.sh <url> <key> [<R> <name>])"`, url & key = literal | `secret:NAME`, R = `read_netrc()` variable, name = cache's. assignment ⊥ `export` (`NIX_CONFIG` already exported) ∴ script's exit status stops job.
- GitLab variable: read netrc = protected File variable; push netrc (`src/cache:V82`) ⊥ used for reads.

## §V INVARIANTS

V81: read secret → ⊥ in `NIX_CONFIG`; `cache-read.sh` prints both setting lines iff url & key both non-empty, else nothing ∴ pipeline w/o protected vars (unprotected ref, fork MR) runs w/o cache, ⊥ empty substituter

V86: read netrc → `cache-read.sh` prints `netrc-file = <file>` after both setting lines; file = `$R`'s own File variable file (runner writes & removes it), `chmod 600` after V87 check; ⊥ copy (⊥ `after_script` to remove one). url, key | `$R` empty (unprotected ref; fork MR stopped earlier by `src/kind/rules:V76`) → prints nothing ∴ ⊥ cache. `https-verify.sh` asks w/ read netrc ∃, else push netrc (`src/cache:V84`); `report` purpose `https cache read (netrc file)`
V87: before handing netrc to nix (chmod, `netrc-file`), `cache-read.sh` checks RESOLVED url starts `https://`; else exit 1, ⊥ `netrc-file`, message names cache & `R`, ⊥ url, ⊥ secret value (pipelith V87). netrc w/ `default` entry (any host) → same refusal ∴ credential scoped to `machine` entries. NAR urls in narinfo ⊥ seen ∴ cache ! serve relative | `https://` ones (doc)

## §T TASKS

id|status|task|cites
T74|x|split from `src/cache` at its ceiling: spec, then code after T75; ⊥ behaviour change|V81
T75|x|pipelith 0.2.2 read netrc: script arg, https check, assignment, verify & `report`; fork guard 1st|V86,V87,`src/kind/rules:V76`
T76|x|review #37: netrc in place, ⊥ copy left on runner|V86
T77|x|review #37: refuse `default` netrc; doc NAR urls & daemon trust|V87
