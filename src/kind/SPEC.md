# SPEC

## §G GOAL

`src/kind` — what every kind shares: `Kind` trait, job rules, timeout, interruptible, `GIT_DEPTH`, nix & cachix parts, secrets.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/kind|what every kind shares: `Kind` trait, job rules, timeout, interruptible, `GIT_DEPTH`, nix & cachix parts, secrets
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape
sib|src/registry|parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors
sib|src/yaml|GitLab dialect for pipelith YAML writer: `$NAME` expressions, `$` escapes
sib|src/system|nix system → runner tags table, built-in + `## runners`
sib|src/pin|image pin table: built-in + `## images`; `image:` lines
sib|src/fetch|OCI registry tag → digest for `pin` via `curl`; ONLY network code
sib|src/pipeline|GitLab pipeline model: root `.gitlab-ci.yml`, per-row file, stages, `workflow: rules`, job-name prefix
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

- kinds = typed Rust data, ⊥ text templates. v1 renders ∀ neutral kind: `gate` `build` `update-pins` `upstream`. ⊥ GitLab-only kinds (pipelith D6).

## §I INTERFACES

- secrets: `CACHIX_AUTH_TOKEN` (`Purpose::CachePush`); `LABLITH_TOKEN` (`Purpose::OpenChange`, project access token, smallest scope that pushes branch & opens MR) = `ArgDefault::Forge` for `token`. masked, protected CI/CD vars only

## §R RESEARCH

id|topic|finding|src
R9|nix cache on GitLab|? proposed cachix (same secret & steps as GitHub ∴ `cache` column same meaning) vs instance cache (Q2). fill before `src/build:T24`|-

## §V INVARIANTS

V11: ∀ job has `timeout:` (default 30 minutes, `timeout` arg overrides)
V12: `interruptible: true` on `gate` & `build` jobs; scheduled bump jobs ⊥ interruptible & take `resource_group` per row
V13: `GIT_DEPTH` set per kind; ⊥ job prints | echoes a variable
V14: cache push skipped when `$CI_MERGE_REQUEST_SOURCE_PROJECT_ID != $CI_PROJECT_ID`
V15: jobs needing secrets run only on protected refs | schedules; secrets only as masked protected CI/CD vars
V31: job names stable across versions (MR pipeline status & docs cite them); rename = breaking change + CHANGELOG entry
V32: ∀ kind declares ∀ secret it reads (`KindSpec.secrets`); `report` lists them per row. fork MR pipeline needs ⊥ secret
V50: ∀ job carries own `rules:`: `gate` & `build` jobs ⊥ run in schedule pipelines; bump jobs run only when `$CI_PIPELINE_SOURCE` ∈ {`schedule`, `web`} & `$LABLITH_ROW == "<id>"` ∴ ⊥ schedule starts gate, ⊥ push starts bump
V53: `GIT_DEPTH` per kind: `gate` `0` (semver step needs tags), `build` `1`, bump kinds `0` (branch push)
V54: `token` arg default (`ArgDefault::Forge`) = `LABLITH_TOKEN`, ⊥ `CI_JOB_TOKEN` ∵ job token cannot open MR. deviates from pipelith "forge built-in job token" wording ∴ note in `docs/KINDS-GITLAB.md` & pipelith issue
V57: nix in job: image-based executor → pinned nix image w/ `NIX_CONFIG` enabling `nix-command flakes`; cachix binary from consumer flake lock (`nix run` on locked input), ⊥ unpinned `nixpkgs#` registry lookup ? exact form pending Q1, Q2

## §T TASKS

id|status|task|cites
T17|.|common job parts: `timeout`, `interruptible`/`resource_group`, `GIT_DEPTH`, nix install via image, cachix w/ fork-MR skip, secret gating|V11,V12,V13,V14,V15
