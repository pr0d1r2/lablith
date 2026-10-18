# SPEC

## §G GOAL

`src` — the tool: one module = one node; dispatch in `main.rs`, re-exports in `lib.rs`.

## §F FEDERATION

dir|owns|⊥owns|tokens
core|facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape|any GitLab logic|-
registry|parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors|grammar (pipelith), kind rules (`src/kind`)|-
yaml|GitLab dialect for pipelith YAML writer: `$NAME` expressions, `$` escapes|writer itself (pipelith)|-
system|nix system → runner tags table, built-in + `## runners`|system names (pipelith)|-
pin|image pin table: built-in + `## images`; `image:` lines|network lookup (`src/fetch`)|-
fetch|OCI registry tag → digest for `pin` via `curl`; ONLY network code|pin table shape (`src/pin`)|-
pipeline|GitLab pipeline model: root `.gitlab-ci.yml`, per-row file, stages, `workflow: rules`, job-name prefix|kind job content (`src/kind` & kind nodes)|-
kind|what every kind shares: `Kind` trait, job rules, timeout, interruptible, `GIT_DEPTH`, nix & cachix parts, secrets|one kind's jobs (kind nodes)|-
gate|kind `gate`: hk gate per system|shared job parts (`src/kind`)|-
build|kind `build`: build, flake check, verify-cache|shared job parts (`src/kind`)|-
bump|kinds `update-pins` & `upstream`: scheduled bump MRs|seeded script text (`src/script`)|-
script|seeded shell scripts shipped in crate (`update-pins.sh`, `open-mr.sh`) & their bats|kinds calling them|-
render|`.lablith` rows → ordered set of files (path, bytes)|writing files (`src/gen`)|-
gen|`gen` & `gen --check`: write or compare rendered files|render (`src/render`)|-
check|`check` totality over `.gitlab-ci.yml` & `.gitlab/ci/`, portability findings, `portable` verb|render (`src/render`)|-
detect|`detect` & `seed --init`: rows from tracked files|detect rules (pipelith)|-
report|`report` text, json, md: rows × systems × pins, secrets, schedules, token|render (`src/render`)|-
cli|clap verbs, flags, exit codes, completions, man page|verb logic (verb nodes)|-

## §N NAV

rel|path|lens
up|.|-
self|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
sib|tests|integration tests driving `lablith` binary, GitLab CI schema harness, golden pipelines, fixtures
sib|dev|`lablith-dev`, `publish = false`: commit rules, README badges, notices, AGENTS step table, setup doc & their drift checks
sib|nix|repo gate: dev shell, guardrails, hk steps, CI, release, dogfood

## §I INTERFACES

- lib: registry, YAML, files, detect via `pipelith`; `lablith` exposes GitLab kind model & render

## §V INVARIANTS

V37: `main.rs` & `lib.rs` dispatch & re-export only, ⊥ logic. lib API = `lib.rs` re-exports only; ∀ node `mod` private; node ⊥ reaches sibling except through its public surface
V38: ∀ node unit-tested 1-to-1 (`<node>/tests.rs`, `rust-mirror`); fixtures `tests/fixtures/<node>/`; ⊥ network, ⊥ clock in unit tests

## §T TASKS

id|status|task|cites
T42|x|AFTER `src/core:T11`: `src/` skeleton rules: `main.rs`/`lib.rs` dispatch only, private `mod`s, `<node>/tests.rs` per node; `src/<node>/mod.rs` scaffold + `use crate::<dep>` build-order edges for `sherd wave`|V37,V38
