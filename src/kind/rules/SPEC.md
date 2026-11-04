# SPEC

## §G GOAL

`src/kind/rules` — `rules:` presets every job takes: change, default branch, bump; fork guard.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
up|src/kind|what every kind shares: `Kind` trait, job rules, timeout, interruptible, `GIT_DEPTH`, nix & cachix parts, secrets
self|src/kind/rules|`rules:` presets every job takes: change, default branch, bump; fork guard

## §R RESEARCH

id|topic|finding|src
R20|fork MR|runs in fork (`CI_PROJECT_ID` = source) or parent ∴ compare w/ `CI_MERGE_REQUEST_PROJECT_ID`; self-hosted: ∀ tags reach owner's runners|GitLab docs, owner 2026-11-03

## §V INVARIANTS

V50: ∀ job carries own `rules:`: `gate` & `build` jobs ⊥ run in schedule pipelines; bump jobs run only when `$CI_PIPELINE_SOURCE` ∈ {`schedule`, `web`} & `$LABLITH_ROW == "<id>"` ∴ ⊥ schedule starts gate, ⊥ push starts bump
V76: ∀ job → 1st rule `$CI_MERGE_REQUEST_SOURCE_PROJECT_ID != $CI_MERGE_REQUEST_PROJECT_ID` never (R20); ⊥ opt-out

## §T TASKS

id|status|task|cites
T66|.|V76 guard on ∀ job; tests|V76,R20
T71|x|split from `src/kind` at its token ceiling; ⊥ behaviour change|V50
