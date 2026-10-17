# SPEC

## §G GOAL

`tests` — integration tests driving `lablith` binary, GitLab CI schema harness, golden pipelines, fixtures.

## §N NAV

rel|path|lens
up|.|-
self|tests|integration tests driving `lablith` binary, GitLab CI schema harness, golden pipelines, fixtures
sib|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
sib|dev|`lablith-dev`, `publish = false`: commit rules, README badges, notices, AGENTS step table & their drift checks
sib|nix|repo gate: dev shell, guardrails, hk steps, CI, release, dogfood

## §R RESEARCH

id|topic|finding|src
R7|GitLab CI schema|? instance GitLab version still unknown (Q4); vendored newest release v19.5.0-ee meanwhile (`tests/schema/VENDORED.md`); `workflow: auto_cancel` ⊥ used (interruptible + project auto-cancel default suffice)|-
R11|schema harness|decided 2026-10-18: ⊥ Rust YAML or JSON-schema crate. `check-jsonschema` 0.37.2 (nixpkgs) validates rendered YAML text vs vendored schema as gate step `gitlab-schema`; hardening asserted on the job model in Rust ∴ ⊥ test-only Rust dep|nixpkgs, `nix/guard/gitlab-schema.sh`
R12|GitLab CI schema source|schema file `app/assets/javascripts/editor/schema/ci.json` in GitLab repo, per release tag; vendor at instance version (Q4) w/ tag + sha256 in `tests/schema/VENDORED.md`|gitlab.com/gitlab-org/gitlab

## §V INVARIANTS

V19: ∀ generated file validates vs vendored GitLab CI JSON schema (`tests/`, recorded GitLab version) w/ 0 findings; offline. `glab ci lint` manual only, ⊥ gate
V39: golden files change only w/ CHANGELOG entry (`.:V25`); `UPDATE_GOLDEN=1` rewrites them, ⊥ default
V61: vendored schema = `tests/schema/ci.json` + `tests/schema/VENDORED.md` (GitLab tag, upstream path, sha256); `schema-pin` step checks hash; bump = 1 reviewed commit
V62: ∀ hardening rule `src/pipeline:V10`–`src/script:V16`, `src/kind:V50`, `src/pipeline:V7` & `src/pipeline:V8` ! own test asserting on the job model (`Job`, `Rule`) of ∀ kind × fixture render (`src/render/tests.rs`), ⊥ string grep
V63: migration-ready: ∀ `.lablith` fixture w/o `## images` & `## runners` parses under actlith's section set (`kind::WORKFLOWS` + `## hand-written`) w/ same rows; tested via pipelith, ⊥ actlith crate dep

## §T TASKS

id|status|task|cites
T18|x|test harness: vendored GitLab CI JSON schema + YAML reader (test-only, §R rows); render ∀ kind × fixture args → 0 findings|V19,R7,R11
T50|x|AFTER T18: vendor schema + `VENDORED.md`, `schema-pin` hk step (push-only) w/ planted hash mismatch|V61,R12
T51|x|AFTER `.:T23`: hardening tests on parsed YAML, 1 per rule; migration-ready test on fixtures|V62,V63

## §B BUGS

id|date|cause|fix
B3|2026-10-18|pre-commit `ctrm fix` rewrote 2 typographic apostrophes in vendored `tests/schema/ci.json` while it was committed (`.ctrm` floor: ascii), so the committed copy no longer hashed to `VENDORED.md`; `ci-schema-pin` (V61) caught it on the next `hk check --all`|`.ctrm` grants `any` to `tests/schema/ci.json`, as to `pkl/Config.pkl`; upstream bytes restored
