# SPEC

## §G GOAL

`src/fetch` — OCI registry tag → digest for `pin` via `curl`; ONLY network code.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/fetch|OCI registry tag → digest for `pin` via `curl`; ONLY network code
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape
sib|src/registry|parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors
sib|src/yaml|GitLab dialect for pipelith YAML writer: `$NAME` expressions, `$` escapes
sib|src/system|nix system → runner tags table, built-in ⊕ `## runners`
sib|src/pin|image pin table: built-in ⊕ `## images`; `image:` lines
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

## §V INVARIANTS

V34: `pin` = sole networked path; writes only `## images` of `.lablith`; `--dry-run` writes ⊥. ⊥ other verb links network code
V59: `pin` failure (network, auth, unknown tag) → exit 1 naming image & tag; `.lablith` untouched (write after ∀ lookups succeed)

## §T TASKS

id|status|task|cites
T30|.|`pin` via image registry HTTP API, `--dry-run`; network isolated in 1 module|`src/pin:V2`
T49|.|AFTER T30: `pin` atomic write, failure exit 1, multi-arch index digest; `curl` stubbed in tests via `PATH` fixture|`src/pin:V58`,V59,V34
