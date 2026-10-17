# SPEC

## §G GOAL

`src/pipeline` — GitLab pipeline model: root `.gitlab-ci.yml`, per-row file, stages, `workflow: rules`, job-name prefix.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/pipeline|GitLab pipeline model: root `.gitlab-ci.yml`, per-row file, stages, `workflow: rules`, job-name prefix
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape
sib|src/registry|parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors
sib|src/yaml|GitLab dialect for pipelith YAML writer: `$NAME` expressions, `$` escapes
sib|src/system|nix system → runner tags table, built-in ⊕ `## runners`
sib|src/pin|image pin table: built-in ⊕ `## images`; `image:` lines
sib|src/fetch|OCI registry tag → digest for `pin` via `curl`; ONLY network code
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
R6|GitLab remote includes|`include: remote:`/`project:`/`template:` & CI/CD components: trust spans repos, lint sees 1 line, fix lands w/o review ∴ rejected (same as actlith reusable wf)|actlith R6

## §V INVARIANTS
V7: generated files: `include:` only `local:`; ⊥ `remote:` ⊥ `project:` ⊥ `template:` ⊥ `component:`
V8: ⊥ `extends:` ⊥ YAML anchors/aliases in generated output; ∀ job expanded
V9: ∀ job name in `.gitlab/ci/<id>.yml` prefixed `<id>:` ∴ ⊥ collision across rows
V10: `workflow: rules` → 1 pipeline per change: MR pipelines; branch pipelines only default branch & tags; schedules. push to MR branch ⊥ runs twice
V51: manual run = `web` pipeline source: `workflow: rules` admit `web`; push/MR kinds run on `web`; bump kinds need `LABLITH_ROW` set in run form (`report` says so)
V52: root `stages:` = fixed list in fixed order (`gate`, `build`, `verify`, `bump`); kind jobs use only these; ⊥ stage w/o job error ∵ GitLab rejects empty pipeline only, ⊥ unused stage. job order inside row via `needs:`, ⊥ stage tricks
V66: root `.gitlab-ci.yml` ⊥ `default:` keyword; ∀ job setting expanded on job (V8) ∵ `default:` silently changes hand-written jobs. deviates from prompt "defaults" wording

## §T TASKS

id|status|task|cites
T16|.|pipeline model: root `.gitlab-ci.yml` (`workflow: rules`, `stages:`, defaults, `include: local:`), per-row file, `<id>:` job prefix; golden bytes tests|`.:V1`,V7,V8,V9,V10
T45|.|AFTER `src/kind:T17`: per-job `rules:` (push/MR vs schedule/web + `LABLITH_ROW`), fixed `stages:`, `needs:`, `GIT_DEPTH` table; 1 golden per pipeline source|`src/kind:V50`,V51,V52,`src/kind:V53`
