# SPEC

## §G GOAL

`src/report` — `report` text, json, md: rows × systems × pins, secrets, schedules, token.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/report|`report` text, json, md: rows × systems × pins, secrets, schedules, token
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
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §V INVARIANTS

V36: `report --format json` carries `schema_version`; field rename | removal bumps it
V64: `LABLITH_TOKEN` = project access token, role Developer, scope `api` ? (MR via API & branch push), expiry ≤ 1 year; `report` & `docs/SECURITY.md` name scope, role & rotation. bump branches (`pins/*`, `upstream/*`) ⊥ protected ∴ their MR pipelines get ⊥ protected var (`src/kind:V15`)
V70: `report` Setup section = numbered checklist derived from `.lablith` alone, ⊥ network: (1) ∀ system in rows: register runner w/ its tags from runner table (built-in + `## runners`); (2) ∀ secret: CI/CD variable, masked & protected; (3) ∀ bump row: schedule on default branch w/ `LABLITH_ROW=<id>`; (4) any bump row → create `LABLITH_TOKEN` per V64; (5) any bump row → default branch protected (`src/kind:V15`). same steps in text, md & json (`setup` array; added field ∴ ⊥ bump, V36). `gen` file set unchanged

## §T TASKS

id|status|task|cites
T29|x|`report` text/json/md: pins, secrets, schedules to create|`src/bump:V17`,I.cmd
T53|x|AFTER T29: `report` token scope/role/rotation & schedule list; `docs/SECURITY.md` token section|V64
T57|.|`report` Setup section text/json/md; 1 fixture per step kind & 1 row-less-secret case|V70,V36,V64
