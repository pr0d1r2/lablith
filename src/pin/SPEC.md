# SPEC

## §G GOAL

`src/pin` — image pin table: built-in + `## images`; `image:` lines.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/pin|image pin table: built-in + `## images`; `image:` lines
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape
sib|src/registry|parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors
sib|src/yaml|GitLab dialect for pipelith YAML writer: `$NAME` expressions, `$` escapes
sib|src/system|nix system → runner tags table, built-in + `## runners`
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

## §R RESEARCH

id|topic|finding|src
R13|built-in nix image|`nixos/nix` newest semver tag 2026-10-18 = `2.35.3`; `HEAD /v2/nixos/nix/manifests/2.35.3` (Accept OCI index + manifest list, anonymous bearer from auth.docker.io) → manifest list `sha256:ff480dbb23511aa1c10710aa068ceea5b93fed9bfce5f5379f3a6b07282cbc7b`, platforms linux/amd64 + linux/arm64; same digest as `latest`|registry-1.docker.io

## §V INVARIANTS

V2: ∀ `image:` = `<name>@sha256:<digest>` + `<name> <version>` comment above `image:` (a trailing one passes 120 columns), from pin table (built-in + `## images`). `gen` ⊥ resolves tags; missing pin → exit 2
V58: built-in image pin table v1 = `nixos/nix` only; digest = multi-arch index digest (covers ∀ linux system), ⊥ per-arch manifest digest

## §T TASKS

id|status|task|cites
T14|x|pin table: built-in + `## images`; missing pin → exit 2; digest + version comment|V2
