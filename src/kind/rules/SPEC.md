# SPEC

## §G GOAL

`src/kind/rules` — `rules:` presets every job takes: change, default branch, bump.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: facade, registry, runners, pins, YAML dialect, pipeline model, kinds, render, verbs, CLI
up|src/kind|what every kind shares: `Kind` trait, job rules, timeout, interruptible, `GIT_DEPTH`, nix & cachix parts, secrets
self|src/kind/rules|`rules:` presets every job takes: change, default branch, bump

## §V INVARIANTS

V50: ∀ job carries own `rules:`: `gate` & `build` jobs ⊥ run in schedule pipelines; bump jobs run only when `$CI_PIPELINE_SOURCE` ∈ {`schedule`, `web`} & `$LABLITH_ROW == "<id>"` ∴ ⊥ schedule starts gate, ⊥ push starts bump

## §T TASKS

id|status|task|cites
T71|x|split from `src/kind` at its token ceiling; ⊥ behaviour change|V50
