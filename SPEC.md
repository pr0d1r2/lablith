# SPEC

## §G GOAL

`lablith` (cmd `lablith`) — GitLab CI pipeline MATERIALIZER: from small `.lablith` (1 row = 1 pipeline part: id, kind, systems, cache, args) RENDER `.gitlab-ci.yml` + `.gitlab/ci/<id>.yml` + seeded scripts; ∀ image digest-pinned; hardening ⊥ optional; CHECK committed files == render (drift) & ∀ CI file declared (totality). GitLab counterpart of `actlith`, both on `pipelith`. purely mechanical: ⊥ LLM, ⊥ guess, ⊥ network (sole exception `lablith pin`, opt-in).

WHY: private/unreleased fleet projects live on self-hosted GitLab; ~⊥ pipelines there (R1). release moves project GitLab → GitHub ∴ CI declaration ! move unchanged: same rows → same CI on both forges; release = file rename + `actlith gen`, ⊥ rewrite.

PAIR: pklith decides WHAT is checked (hk gate); lablith decides WHERE & WHEN GitLab runs it. lablith ⊥ adds lint steps.

NAME: (Git)lab + lith, as actlith = act(ions) + lith (V23).

FIRST BUILT: flake dev shell via direnv w/ nix-hk's hk, then hk installs hooks (`nix:T2`, `nix:T3`) — before ANY other file, spec format files included. then guardrails, 1 check per commit, then product code.

SOURCES: `SPEC-PROMPT.md` (decisions D1–D14, learnings 2026-10-17), pipelith `docs/KINDS.md` (neutral kinds), `docs/KINDS-GITLAB.md` (GitLab rendering). this spec wins where they differ.

## §F FEDERATION

dir|owns|⊥owns|tokens
src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI|repo gate, dev tooling, integration harness|-
tests|integration tests driving `lablith` binary, GitLab CI schema harness, golden pipelines, fixtures|unit tests (beside their module)|-
dev|`lablith-dev`, `publish = false`: commit rules, README badges, notices, AGENTS step table, setup doc & their drift checks|anything a consumer runs|-
nix|repo gate: dev shell, guardrails, hk steps, CI, release, dogfood|the tool|-

## §N NAV

rel|path|lens
up|-|-
self|.|-

## §C CONSTRAINTS

- upstream fixes: lablith needs ∃ thing from pipelith (kind, detect API, YAML writer, row rule) → file pipelith issue & lablith commit | task carries `Depends-on: pr0d1r2/pipelith#<n>` trailer; loop drives pipelith issue, lablith chain follows. ⊥ local workaround hiding the gap, ⊥ fork of pipelith code

- deterministic core: same `.lablith` + same lablith version → byte-identical output. ⊥ LLM, ⊥ heuristic guess, ⊥ clock | rng | hostname in output, ⊥ network except `pin` (V1).
- lang: Rust edition 2024, stable, MSRV 1.95 = fleet pin (`nixpkgs-lock` → nixos-26.05). MIT. `unsafe_code = "forbid"`.
- layout: ONE published crate `lablith` (lib + bin `lablith` ∴ `cargo install lablith` gives cmd); `dev/` = `lablith-dev`, `publish = false`. module = dir + `mod.rs` = node.
- built on `pipelith = "0.1"` from crates.io (≥ 0.1.1), ⊥ git dep (blocks publish, needs `deny.toml` allow-git & nix `outputHashes`). pipelith owns registry grammar, YAML writer, generated-file set, neutral kinds, nix systems, detect. lablith owns only GitLab: pipeline model, runner tags, image pins, hardening, layout, scripts, CLI.
- deps minimal; ∀ dep ! justified in §R & owned by ONE facade node. `clap` (derive) → `src/cli`. JSON-schema validation & YAML reading = test-only deps, each w/ §R row.
- runtime deps: `git`; `curl` for `pin` & seeded `open-mr.sh` (⊥ HTTP crate). bump kinds run `nix develop --command bash scripts/lablith/....sh` ∴ consumer dev shell ! provide `lablith` (pklith catalog row, pklith issue #70 shape).
- ⊥ push | PR to another repo w/o explicit approval. agent ⊥ publishes.

## §I INTERFACES

- cmd: `lablith gen [--root DIR] [--check]` → write `.gitlab-ci.yml`, `.gitlab/ci/<id>.yml` ∀ row, `scripts/lablith/*.sh` kinds need. `--check` writes ⊥, exit 1 naming each stale | missing file
- cmd: `lablith check [--root DIR]` → totality: `.gitlab-ci.yml` & ∀ file ∈ `.gitlab/ci/` generated | `## hand-written`; ⊥ listed path missing; ⊥ generated file w/o row; + portability findings (D9)
- cmd: `lablith detect [--root DIR]` → proposed `## workflows` rows from tracked files (pipelith detect), stdout
- cmd: `lablith seed --init [--root DIR]` → first `.lablith` from `detect`; ⊥ over existing file
- cmd: `lablith report [--root DIR] [--format text|json|md]` → row × system × image pin; secrets per row; pipeline schedules to create (cron, branch, `LABLITH_ROW` value). json versioned
- cmd: `lablith pin [--root DIR] [--dry-run]` → resolve ∀ image tag → digest via OCI distribution API (`curl` subprocess, anonymous bearer token, `HEAD /v2/<name>/manifests/<tag>`, digest of multi-arch index); write `## images`. ONLY networked verb
- cmd: `lablith portable [--root DIR]` → exit 1 naming each row | system | arg key actlith cannot render
- cmd: `lablith completions <shell>` · `lablith man`
- exit: 0 ok · 1 finding · 2 usage | unreadable | unknown kind/key/system | newer format
- file: `.lablith` → pipelith registry grammar: `format 1`; `## workflows` `id|kind|systems|cache|args` (neutral, = `.actlith`); `## images` `image|version|digest` (overrides built-in pin table); `## runners` `system|tags` (overrides built-in system → tags table); `## hand-written` `path|reason`. `#` comments, `-` empty, `\|` literal pipe. `## images` & `## runners` GitLab-only, ⊥ portability findings
- file: `.gitlab-ci.yml` → `workflow: rules`, `stages:`, defaults, 1 `include: local:` per row & per hand-written file
- file: `.gitlab/ci/<id>.yml` → jobs of 1 row, ∀ job name prefixed `<id>:`
- file: generated header `# generated by lablith gen from .lablith; do not edit` = `files::header("lablith", ".lablith")`
- kinds: GitLab rendering per kind → `docs/KINDS-GITLAB.md` (human) & kind nodes §V (law)
- docs: README (first line says "GitLab CI") → `docs/INTEGRATION.md`, `docs/REGISTRY.md`, `docs/RELEASE.md` (GitLab → GitHub migration), `docs/KINDS-GITLAB.md`, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT, `docs/LLM-DISCLAIMER.md`, `docs/THIRD-PARTY-NOTICES.md` (generated)

## §R RESEARCH

id|topic|finding|src
R1|fleet survey|52 checkouts w/ self-hosted GitLab remote, 46 w/o GitHub remote. 2 w/ `.gitlab-ci.yml`, neither runs hk: 1 = lint tools in `nix develop` on `nixos/nix:latest` (unpinned); 1 = Rails + Postgres service + `include: template:`. 0 render CI from config. runner fleet ⊥ visible from checkouts|local checkouts 2026-10-16
R2|name|`lablith` free 2026-10-16: crates.io 404, ⊥ `pr0d1r2/lablith`. rejected `glablith` (echoes `glab` CLI), `gitlith` (says git, ⊥ GitLab)|crates.io API, `gh repo view`
R3|siblings|pipelith 0.1.1 on crates.io, std only. actlith 0.1.0 on crates.io; actlith `main` takes `registry` `kind` `system` `yaml` `files` `detect` from pipelith, goldens byte-identical (actlith PRs #7–#14)|../pipelith, ../actlith 2026-10-17
R10|config file name|? pipelith open question 1; blocks `docs/RELEASE.md` only (Q3)|../pipelith

## §V INVARIANTS

V1: deterministic: same `.lablith` + same version → byte-identical files. ⊥ clock, ⊥ host, ⊥ env, ⊥ network in `gen`|`check`|`detect`|`report`|`portable`
V4: `## workflows` row valid in `.lablith` ↔ valid in `.actlith` (same columns, same `Row::check`, `Extras` empty in v1)
V20: ⊥ private name (runner tag, registry, host, project path) of self-hosted instance in committed file; built-in runner tags neutral (nix system + `nix`)
V23: crate & bin `lablith`. rejected names → R2
V24: ∀ kind ∈ pipelith `docs/KINDS.md` ! row in `docs/KINDS-GITLAB.md` & §V coverage & §T row; parity test (kind list code order, args key/required/default) ⊥ drift. doc ≠ spec = bug
V25: emitted bytes change for same inputs → ≥ minor version + CHANGELOG naming files consumers see change; golden diff w/o changelog touch → red
V27: GitLab-only product features (security templates, Auto DevOps, environments, review apps, services) ⊥ rendered; need → `## hand-written`
V49: ⊥ `import` verb in v1 (R1: 2 hand-written pipelines, neither kind-shaped); existing pipelines → `## hand-written` | rows by hand
V67: `docs/RELEASE.md` steps, 1 reviewed commit: rename `.lablith` → `.actlith`; drop `## images` & `## runners`; rewrite `## hand-written` paths to `.github/workflows/`; `actlith gen`; delete `.gitlab-ci.yml` & `.gitlab/ci/` & `scripts/lablith/`. then outside repo: delete GitLab schedules & `LABLITH_TOKEN`, create GitHub secrets per `actlith report`

## §T TASKS

id|status|task|cites
T10|x|federate `SPEC.md` w/ sherd: nodes `src/core`, `src/*`, `tests`, `dev`, `nix`; `.context-limits` ~12% over measured. `src/<node>/mod.rs` + build-order edges land w/ `src:T42`|-
T23|x|`docs/KINDS-GITLAB.md` + parity test vs code & kind nodes (port actlith `nix:T33`)|V24
T34|x|human docs: README (GitLab CI first line) → INTEGRATION, REGISTRY, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT; badges after pages exist|I.docs
T35|.|`docs/RELEASE.md` per V67 — blocked on Q3|V67,`tests:V63`,R10
T37|.|pklith: `lablith-gen-check` catalog row & `.lablith` type (join \| follow pklith #70) — PR ! user approval|I.nix
T38|.|release `0.1.0`: version bump via PR; owner runs `cargo release` tag, publish, push from `main`; agent ⊥ publishes|V23
T39|.|fleet rollout: lablith mirror, then private projects 1 at a time: `detect` → reviewed diff → explicit approval|R1
T40|.|paradigm gap round (separate): name guard extension (D12), private projects in registry (D13) — proposal only|V20
T46|x|pipelith issues: `token` default wording per forge → pr0d1r2/pipelith#9 (docs only). id charset ⊥ needed: pipelith already refuses non-file-stem ids|`src/registry:V55`,`src/kind:V54`

## §B BUGS

id|date|cause|fix
B2|2026-10-18|`src/registry:V55` demanded id charset `[a-z0-9][a-z0-9-]*`, stricter than pipelith's `[A-Za-z0-9._-]` ∴ a row valid in `.actlith` refused here, breaking V4; written w/o reading `RawRow::from_cells`|`src/registry:V55` rewritten: pipelith's charset, ⊥ narrowed; test per rendered form
