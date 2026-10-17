# SPEC

## §G GOAL

`src/registry` — parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/registry|parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape
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

## §V INVARIANTS

V28: unknown kind | arg key | system | section, missing required arg, `x86_64-darwin` → exit 2 naming it & fix; ⊥ silent default
V29: `.lablith` ! carry `format` line; newer than supported → exit 2 naming both versions, ⊥ best-effort parse. missing `.lablith` → `gen`|`check`|`report`|`portable` exit 2, ⊥ empty render. missing optional section (`## images`, `## runners`, `## hand-written`) = empty
V55: row `id` charset = pipelith's (`RawRow::from_cells`: `[A-Za-z0-9._-]`, ⊥ leading dot) & lablith ⊥ narrows it (`.:V4`). each such id renders safely as file stem, `<id>:` job prefix, `resource_group` & quoted `rules:` string; test per form

## §T TASKS

id|status|task|cites
T12|.|registry: `.lablith` schema = `kind::WORKFLOWS` + `## images`, `## runners`, `## hand-written` SectionSpecs; fixtures 1-to-1|`.:V4`,I.file
T43|.|AFTER T12: registry errors: unknown section/kind/key/system, `format` newer, missing file, missing optional section; 1 fixture each|V28,V29
