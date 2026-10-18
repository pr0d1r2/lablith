# SPEC

## §G GOAL

`src/system` — nix system → runner tags table, built-in + `## runners`.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/system|nix system → runner tags table, built-in + `## runners`
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape
sib|src/registry|parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors
sib|src/yaml|GitLab dialect for pipelith YAML writer: `$NAME` expressions, `$` escapes
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

## §I INTERFACES

- runners: default tags = nix system name + `nix` (e.g. `x86_64-linux`, `nix`); fit the owner's fleet as registered (R8)

## §R RESEARCH

id|topic|finding|src
R8|runner fleet|decided (Q1): self-hosted `x86_64-linux` NixOS hosts, `shell` executor on host nix daemon, modelled on a runner already on the instance. registered w/ tags `nix` + system ∴ built-in tags fit, ⊥ `## runners` row. `image:` ignored under shell, honoured under Docker ∴ 1 render serves both. a host w/o tag `nix` (e.g. a production box) ⊥ takes lablith jobs|owner 2026-10-18

## §V INVARIANTS

V56: system w/ ⊥ entry in runner table (built-in + `## runners`) → exit 2 naming system & `## runners` fix; ⊥ untagged job (would land on any shared runner)
V72: `## runners` row `<system>|-` = SKIP: ⊥ runner for system on this forge ∴ its per-system jobs (`gate:<system>`, `build:<system>`) ⊥ rendered, build `check` evaluates it like an untagged system, `verify-cache` omits it; a job that must run on it (bump row's system, system-free jobs on `x86_64-linux`) → exit 2 naming the skip; row w/ every system skipped → exit 2; `report` marks it skipped. rows stay = `.actlith` (`.:V4`); SKIP ⊥ portability finding (GitLab-only section)

## §T TASKS

id|status|task|cites
T15|x|runner table: system → tags, built-in neutral + `## runners`; own `SystemError` message for unsupported system|`.:V20`,R8
T47|x|AFTER T15: unmapped system → exit 2 w/ `## runners` fix|V56
T59|.|`## runners` SKIP (`<system>\|-`): table, gate, build (check eval, verify-cache), bump & system-free refusal, all-skipped row refusal, report text/json; test per case; dogfood `.lablith` skips `aarch64-linux` & `aarch64-darwin`|V72,V56,`.:V4`
