# SPEC

## §G GOAL

`src/cache` — `## caches` on GitLab: read, push, verify, secrets.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/cache|`## caches` on GitLab: read, push, verify, secrets
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape
sib|src/registry|parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors
sib|src/yaml|GitLab dialect for pipelith YAML writer: `$NAME` expressions, `$` escapes
sib|src/system|nix system → runner tags table, built-in + `## runners`
sib|src/pin|image pin table: built-in + `## images`; `image:` lines
sib|src/fetch|OCI registry tag → digest for `pin` via `curl`; ONLY network code
sib|src/pipeline|GitLab pipeline model: root `.gitlab-ci.yml`, per-row file, stages, `workflow: rules`, job-name prefix
sib|src/kind|what every kind shares: `Kind` trait, job rules, timeout, interruptible, `GIT_DEPTH`, nix & cachix parts, secrets
sib|src/gate|kind `gate`: hk gate per system
sib|src/build|kind `build`: build, flake check, verify-cache
sib|src/bump|kinds `update-pins` & `upstream`: scheduled bump MRs
sib|src/script|seeded shell scripts shipped in crate (`update-pins.sh`, `open-mr.sh`) & their bats
sib|src/render|`.lablith` rows → ordered set of files (path, bytes)
sib|src/gen|`gen` & `gen --check`: write or compare rendered files
sib|src/check|`check` totality over `.gitlab-ci.yml` & `.gitlab/ci/`, portability findings, `portable` verb
sib|src/detect|`detect` & `seed --init`: rows from tracked files
sib|src/report|`report` text, json, md: rows × systems × pins, secrets, schedules, token
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- deps: `src/core` (pipelith `cache`), `src/script`. section grammar, kinds & every row check = pipelith's (its `src/cache` V82–V85); lablith ⊥ re-checks, ⊥ narrows (`.:V4`).
- a secret value ⊥ in `.lablith`, ⊥ in a rendered file, ⊥ in a job log: lines name the variable, seeded scripts read it (`secret:NAME` arg → `${!NAME}`), ⊥ echo url.

## §I INTERFACES

- schema: `cache::CACHES` in `.lablith` schema; `Caches::from_document` once per render; `Row::check_with` per row; cache steps only from `Checked.cache` (`BinaryCache`).
- read: url & key literal → `NIX_CONFIG` lines (= `nix_config()`); ∃ secret → 1st line after runner check `export NIX_CONFIG="$NIX_CONFIG$(bash scripts/lablith/cache-read.sh <url> <key>)"`, arg = literal | `secret:NAME`.
- push: `ReadOnly` → ⊥ step; `Forge` → `cachix.sh <name>` as before; `Secret(N)` → `cache-push.sh <kind> <name|url> N <any|default> <command> [installable…]`; https installable: `gate` `.#devShells.<system>.default.inputDerivation`, `build` `.#packages.<system>.<package>` (as actlith gate).
- verify (`build`): cachix → `verify-cache.sh` as before; https → `https-verify.sh <url> <N|-> <package> <system>...`.
- GitLab variables: url & key secret = masked, protected Variable; cachix push = masked, protected Variable; https push = protected File variable (netrc), its url Variable ! masked.

## §V INVARIANTS

V80: `.lablith` w/o `## caches` → every rendered byte as before (built-in cachix via `Push::Forge`); `## caches` row shadows built-in name
V81: read secret → ⊥ in `NIX_CONFIG`; `cache-read.sh` prints both setting lines iff url & key both non-empty, else nothing ∴ pipeline w/o protected vars (unprotected ref, fork MR) runs w/o cache, ⊥ empty substituter
V82: `cache-push.sh` pushes only when secret non-empty, url non-empty, ⊥ fork MR (`src/kind/rules:V76` condition) & branch rule holds (`default` → `$CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH`); cachix → command under `cachix watch-exec` w/ token from secret; https → each installable ! evaluate (else exit 1 naming it: flake ! have it), then `umask 077`, netrc → 0600 temp file removed on exit, `nix copy --option netrc-file <file> --to <url> <installable>…` BEFORE command ∴ pushed even when gate fails; ⊥ echo url; else command alone. url ! `https://` (else exit 1, generic message, before netrc use); `nix copy` failure = warning ⊥ url, command still runs, exit = command's (`build` caught by verify-cache)
V83: `build` w/ push secret → `default`; `gate` → `any`. `ReadOnly` `build` = pipelith refusal
V84: `https-verify.sh` asks `<url>/<hash>.narinfo` per system (netrc when given), ⊥ cachix host, ⊥ echo url; exit 1 on ∃ miss; url ! `https://` before netrc sent
V85: `report` secrets per row: push secret (`Forge` → `CACHIX_AUTH_TOKEN`) & ∀ `read_secrets()`, each w/ purpose

## §T TASKS

id|status|task|cites
T72|x|`## caches` in schema, `check_with`, read/push/verify rendering, 3 seeded scripts, report secrets; test per V; `.lablith` w/o section byte-identical|V80,V81,V82,V83,V84,V85,`.:V25`
T73|.|review on #35: best-effort https push, `https://`-only url before any netrc, bash 3.2-safe empty arrays|V82,V84
