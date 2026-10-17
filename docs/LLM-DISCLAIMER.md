# Built by an LLM, deliberately and in the open

This repository (code, spec, tests and prose) was written by
[Claude Code](https://claude.com/claude-code) running Anthropic's **Claude
Opus 5.5**. Every commit carries a `Co-Authored-By: Claude Opus 5.5`
trailer, which `git log --format=%B | grep -c Co-Authored-By` will
confirm against `git rev-list --count HEAD`. A human owns every decision,
reviews every diff, and is accountable for what ships.

## Why say it at all

An LLM writes plausible code, and plausible is not correct. A reader who
does not know how a repository was produced cannot calibrate how hard to
look at it. lablith exists to replace hand-written CI with files rendered
from one place and checked against it, so the standard here is the same:
**a claim in prose is a defect until something executes it.**

## The method is spec-driven development

[`SPEC.md`](../SPEC.md) and one `SPEC.md` per node below it are the law,
not a description written afterwards. Together they carry
**69 invariants** (the `V` rows: what must stay true), `T` tasks (what is
decided and what is not), **3 bugs** in their `B` tables (each paired
with the rule that now catches it), and `R` research (the measurements
the constraints rest on). A rule and its checker land in the same commit.

## The guardrails are git hooks that also run on CI

Entering the dev shell (`nix develop`, or `direnv allow`) points git at
the tracked hooks, which run [hk](https://github.com/jdx/hk) against one
definition of the gate in [`hk.pkl`](../hk.pkl). Each check was laid in
its own commit, and the commit that brings it records the planted
violation it refused. The GitHub workflow and the GitLab pipeline run
that same definition, line coverage has a floor of 99.02% that may only
rise,
`unsafe` is
forbidden, and the only command meant to touch the network
is `lablith pin`.

## What a reader should check

1. **Does the gate run for you?** `nix develop`, then `hk check --all`.
2. **Does the bug table look like a real log or a curated one?**
3. **Do the invariants have runners?** An invariant that nothing executes
   is exactly the defect this project exists to remove.

## Accountability

The human named in [`LICENSE`](../LICENSE) is responsible for this code,
including the parts a model wrote and the parts nobody caught. "The LLM
wrote it" explains provenance; it never transfers responsibility.
