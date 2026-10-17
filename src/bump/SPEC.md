# SPEC

## §G GOAL

`src/bump` — kinds `update-pins` & `upstream`: scheduled bump MRs.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/bump|kinds `update-pins` & `upstream`: scheduled bump MRs
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
sib|src/script|seeded shell scripts shipped in crate (`update-pins.sh`, `open-mr.sh`) & their bats
sib|src/render|`.lablith` rows → ordered set of files (path, bytes)
sib|src/gen|`gen` & `gen --check`: write or compare rendered files
sib|src/check|`check` totality over `.gitlab-ci.yml` & `.gitlab/ci/`, portability findings, `portable` verb
sib|src/detect|`detect` & `seed --init`: rows from tracked files
sib|src/report|`report` text, json, md: rows × systems × pins, secrets, schedules, token
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §I INTERFACES

- schedules: GitLab UI/API, ⊥ YAML. scheduled jobs gated `$CI_PIPELINE_SOURCE == "schedule"` & `$LABLITH_ROW == "<id>"`

## §V INVARIANTS

V17: scheduled kinds: ∀ job gated `$CI_PIPELINE_SOURCE == "schedule"` & `$LABLITH_ROW == "<id>"`; `report` lists ∀ schedule (cron, branch, var)
V33: bump kinds idempotent: branch exists → exit 0, ⊥ 2nd MR

## §T TASKS

id|status|task|cites
T21|.|kind `update-pins` (schedule gate, `LABLITH_ROW`, `resource_group`, opens MR)|`src/kind:V12`,`src/kind:V15`,V17,`.:V24`
T22|.|kind `upstream`|`src/kind:V12`,`src/kind:V15`,V17,`.:V24`
