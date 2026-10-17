# SPEC

## §G GOAL

`src/core` — facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
self|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules, converts its errors to `.lablith:N:` shape
sib|src/registry|parse `.lablith`: `## workflows`, `## images`, `## runners`, `## hand-written`; row & file errors
sib|src/yaml|GitLab dialect for pipelith YAML writer: `$NAME` expressions, `$` escapes
sib|src/system|nix system → runner tags table, built-in ⊕ `## runners`
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

## §C CONSTRAINTS

- ONE facade node names pipelith: `src/core` re-exports `pipelith::{detect, files, kind, registry, system, yaml}`; first in build order (`src/core:V3`).

## §I INTERFACES

- errors: `.lablith:N: what` then `  fix: ...` (actlith shape); pipelith `RegistryError` & `KindError` converted at `src/core` facade

## §R RESEARCH

id|topic|finding|src
R4|pipelith API|`registry::parse(FILE, text, &Schema{sections})`; `kind::WORKFLOWS` + forge `SectionSpec{name,columns,required,key}`; `kind::RawRow::from_cells`; `registry::{required,optional,list,args}`. `kind::{GATE,BUILD,UPDATE_PINS,UPSTREAM}: KindSpec{name,schema,default_systems,secrets}`; `Row::check(spec, raw, &EXTRAS)` first fault order: systems, cache, args, `timeout`, row rules. `kind::Extras{kinds,keys}`, `kind::portable(&raw,&extras)` → `Finding`s. `yaml::document(&dialect, comments, entries)`, `impl yaml::Dialect{expr, escape}`. `files::{File,write,check,Stale,header}`. `detect::{detect,row_line,text_with,seed_init_text,header,FIRST_ROW}`|../actlith `src/core`

## §V INVARIANTS
V3: `pipelith::` path only in `src/core`; ∀ other module imports via facade

## §T TASKS

id|status|task|cites
T11|.|`src/core` facade: re-export pipelith modules; convert `RegistryError`/`KindError` → `.lablith:N:` shape; `header("lablith", ".lablith")` + test pinning any `concat!` copy|V3,I.errors
