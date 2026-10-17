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
R7|GitLab CI schema|? GitLab version of instance → schema version to vendor & `workflow: auto_cancel` availability (Q4). fill before `src/bump:T21`|-
R11|test-only deps|? YAML reader + JSON-schema validator for V19 harness. candidates: `jsonschema` (draft-07 ok), YAML via `serde_norway` \| `saphyr` (`serde_yaml` archived). pick by: maintained, MSRV ≤ 1.95, `cargo deny` clean, dev-dependency only. fill before T18|-
R12|GitLab CI schema source|schema file `app/assets/javascripts/editor/schema/ci.json` in GitLab repo, per release tag; vendor at instance version (Q4) w/ tag + sha256 in `tests/schema/VENDORED.md`|gitlab.com/gitlab-org/gitlab

## §V INVARIANTS
V19: ∀ generated file validates vs vendored GitLab CI JSON schema (`tests/`, recorded GitLab version) w/ 0 findings; offline. `glab ci lint` manual only, ⊥ gate
V39: golden files change only w/ CHANGELOG entry (`.:V25`); `UPDATE_GOLDEN=1` rewrites them, ⊥ default
V61: vendored schema = `tests/schema/ci.json` + `tests/schema/VENDORED.md` (GitLab tag, upstream path, sha256); `schema-pin` step checks hash; bump = 1 reviewed commit
V62: ∀ hardening rule `src/pipeline:V10`–`src/script:V16`, `src/kind:V50` ! own test asserting on parsed YAML of ∀ kind × fixture render, ⊥ string grep
V63: migration-ready: ∀ `.lablith` fixture w/o `## images` & `## runners` parses under actlith's section set (`kind::WORKFLOWS` + `## hand-written`) w/ same rows; tested via pipelith, ⊥ actlith crate dep

## §T TASKS

id|status|task|cites
T18|.|test harness: vendored GitLab CI JSON schema + YAML reader (test-only, §R rows); render ∀ kind × fixture args → 0 findings|V19,R7,R11
T50|.|AFTER T18: vendor schema + `VENDORED.md`, `schema-pin` hk step (push-only) w/ planted hash mismatch|V61,R12
T51|.|AFTER `.:T23`: hardening tests on parsed YAML, 1 per rule; migration-ready test on fixtures|V62,V63
