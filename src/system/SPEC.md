# SPEC

## §G GOAL

`src/system` — nix system → runner tags table, built-in ⊕ `## runners`.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/system|nix system → runner tags table, built-in ⊕ `## runners`
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape
sib|src/registry|parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors
sib|src/yaml|GitLab dialect for pipelith YAML writer: `$NAME` expressions, `$` escapes
sib|src/pin|image pin table: built-in ⊕ `## images`; `image:` lines
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

## §I INTERFACES

- runners: default tags = nix system name + `nix` (e.g. `x86_64-linux`, `nix`) ? pending R-row on instance fleet (Q1)

## §R RESEARCH

id|topic|finding|src
R8|runner fleet|? executor (Docker + pinned `nixos/nix` \| shell on NixOS), arches (`aarch64-linux`? `aarch64-darwin`?), default tags (Q1). fill before T15, `src/build:T24`|-

## §V INVARIANTS
V56: system w/ ⊥ entry in runner table (built-in ⊕ `## runners`) → exit 2 naming system & `## runners` fix; ⊥ untagged job (would land on any shared runner)

## §T TASKS

id|status|task|cites
T15|.|runner table: system → tags, built-in neutral ⊕ `## runners`; own `SystemError` message for unsupported system|`.:V20`,R8
T47|.|AFTER T15: unmapped system → exit 2 w/ `## runners` fix|V56
